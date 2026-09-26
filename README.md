# Plataforma de análise de competições de programação

Este repositório reúne uma API analítica em Rust/Axum, uma interface web em
JavaScript e um banco de dados PostgreSQL. A stack pode ser executada
integralmente com Docker ou, durante o desenvolvimento, com backend e frontend
iniciados diretamente no host.

Os comandos deste documento pressupõem um terminal Linux ou WSL e devem ser
executados a partir da raiz do repositório, salvo indicação em contrário.

## Componentes

| Diretório | Responsabilidade | Tecnologias principais |
| --- | --- | --- |
| `backend/` | API HTTP, regras de aplicação e persistência | Rust 1.98.0, Axum, SQLx e PostgreSQL |
| `frontend/` | Dashboard e testes da interface | JavaScript, Vite, Bun 1.3.14 e Playwright |
| `infra/` | Proxy reverso e HTTPS local | Nginx 1.30.4 |

## Pré-requisitos

Para executar a stack completa, bastam:

- Docker Engine;
- Docker Compose v2.

O desenvolvimento sem os contêineres de aplicação também requer:

- Rust 1.98.0;
- Bun 1.3.14;
- `make`, para os comandos de cobertura e análise do backend;
- Docker ativo, para os testes BDD que usam Testcontainers.

## Configuração

O Compose lê as credenciais e a URL do PostgreSQL de um arquivo `.env` na raiz.
Crie-o, sem adicioná-lo ao controle de versão, com valores próprios para o
ambiente local. Por exemplo:

```dotenv
POSTGRES_USER=maratona
POSTGRES_PASSWORD=altere-esta-senha
POSTGRES_DB=maratona
DATABASE_URL=postgres://maratona:altere-esta-senha@db:5432/maratona
```

O hostname `db` é resolvido apenas na rede do Compose. Ao iniciar o backend
diretamente no host, use `127.0.0.1` na `DATABASE_URL`.

## Inicialização rápida com Docker

Valide a configuração e construa os três serviços:

```bash
docker compose config --quiet
docker compose up -d --build
docker compose ps
```

Após a inicialização, os serviços ficam disponíveis em:

- frontend: <https://localhost>;
- API por meio do proxy: <https://localhost/api/>;
- API diretamente: <http://localhost:8000/>;
- PostgreSQL: `localhost:5432`.

O acesso por HTTP à interface é redirecionado para HTTPS. Como o certificado
local é autoassinado, o navegador pode exibir um aviso de segurança. A API
executa automaticamente as migrações embutidas antes de aceitar requisições.

### Operação dos contêineres

```bash
# Acompanhar todos os serviços
docker compose logs -f

# Acompanhar um serviço específico
docker compose logs -f api
docker compose logs -f web
docker compose logs -f db

# Reconstruir somente uma aplicação
docker compose up -d --build api
docker compose up -d --build web

# Validar a configuração do Nginx em execução
docker compose exec web nginx -t

# Parar a stack e preservar os volumes
docker compose down
```

Para apagar também o banco persistido e o certificado local, use:

```bash
docker compose down --volumes
```

Esse último comando é destrutivo: ele remove os volumes `pgdata` e `tls-certs`.

## Compilação e execução local

### Backend

Para compilar sem ou com otimizações:

```bash
cd backend
cargo build --locked
cargo build --release --locked
```

É possível manter apenas o PostgreSQL no Compose e executar a API no host:

```bash
docker compose up -d db
cd backend
export DATABASE_URL=postgres://maratona:altere-esta-senha@127.0.0.1:5432/maratona
cargo run --locked
```

A API passa a responder em <http://127.0.0.1:8000>. A variável
`DATABASE_URL` precisa estar presente no ambiente do processo.

### Frontend

Com a API disponível na porta 8000:

```bash
cd frontend
bun ci
API_PROXY_TARGET=http://127.0.0.1:8000 bun run dev
```

O servidor de desenvolvimento fica normalmente em <http://localhost:5173> e
encaminha as requisições iniciadas por `/api` para o backend. Para gerar e
inspecionar o bundle de produção:

```bash
bun run build
bun run preview
```

O resultado da compilação é gravado em `frontend/dist/`.

## Testes e qualidade

### Backend

Execute os comandos dentro de `backend/`:

```bash
# Verificação rápida de compilação
cargo check --all-targets --all-features --locked

# Formatação e análise estática
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked

# Testes unitários
cargo test --lib --locked

# Cenários BDD/Gherkin
cargo test --test cucumber --locked -- --color never

# Toda a suíte Rust
cargo test --locked
```

Os cenários BDD exigem que o Docker esteja ativo. A suíte mantém um único
PostgreSQL efêmero via Testcontainers, prepara uma base-modelo uma vez e
compartilha uma cópia somente para leitura entre cenários consultivos. Cenários
que alteram a persistência recebem bases isoladas clonadas desse modelo.

Para instalar as ferramentas auxiliares e gerar métricas e cobertura:

```bash
make install-analysis-tools
make quality
make coverage-unit
make coverage-bdd
make coverage-all
```

Os relatórios ficam em `backend/target/quality/` e
`backend/target/coverage/{unit,bdd,all}/`. Cada diretório de cobertura contém
um resumo JSON, um arquivo LCOV e um relatório HTML.

### Frontend

Execute os comandos dentro de `frontend/` após `bun ci`:

```bash
# ESLint e Stylelint
bun run quality

# Prettier, build e testes unitários
bun run format:check
bun run build
bun run test:unit
bun run test:coverage

# Verificações de qualidade, build e cobertura em sequência
bun run check
```

A cobertura JavaScript é gravada em `frontend/coverage/lcov.info`.

### Testes de navegador e de contrato

Instale o Chromium gerenciado pelo Playwright uma vez e execute os cenários E2E
com respostas da API simuladas:

```bash
cd frontend
bun run test:e2e:install
bun run test:e2e
```

Os testes de contrato consultam uma API real:

```bash
CONTRACT_API_URL=http://127.0.0.1:8000 bun run test:contract
```

Para o smoke test full-stack, inicie banco e API, carregue a fixture
determinística e então execute o Playwright:

```bash
# Na raiz do repositório
docker compose up -d --build db api
curl --fail http://127.0.0.1:8000/organizers/options

docker compose exec -T db sh -c \
  'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -v ON_ERROR_STOP=1' \
  < backend/tests/fixtures/api_bdd.sql

# Em frontend/
cd frontend
bun ci
bun run test:e2e:install
CONTRACT_API_URL=http://127.0.0.1:8000 bun run test:contract
API_PROXY_TARGET=http://127.0.0.1:8000 bun run test:e2e:fullstack
```

Se a verificação com `curl` ocorrer antes de a API terminar a inicialização,
aguarde alguns segundos e repita-a antes de carregar a fixture.

A fixture começa com `TRUNCATE` e deve ser aplicada somente a um banco local e
descartável. Ela nunca deve ser carregada em uma base com dados que precisem ser
preservados.

## Imagens Docker isoladas

O Compose é o caminho recomendado, mas as imagens também podem ser construídas
separadamente:

```bash
docker build -t competition-api ./backend
docker build -f frontend/Dockerfile -t competition-web .
```

## Integração contínua

O workflow em `.github/workflows/ci.yml` reproduz as verificações principais:
formatação, análise estática, cobertura e BDD do backend; qualidade, build,
cobertura, contratos e Playwright no frontend; smoke test full-stack; e build
das imagens Docker.

## Documentação adicional

- [`backend/README.md`](backend/README.md): arquitetura e convenções da API;
- [`frontend/README.md`](frontend/README.md): detalhes da toolchain e dos testes;
- [`infra/README.md`](infra/README.md): proxy, HTTPS e HTTP/2 locais.
