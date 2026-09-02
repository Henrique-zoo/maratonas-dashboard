import { escapeHtml } from './ui.js';

function selectedValueList(selectedValue) {
  if (Array.isArray(selectedValue)) {
    return selectedValue.map((value) => String(value)).filter(Boolean);
  }

  if (selectedValue === undefined || selectedValue === null || selectedValue === '') {
    return [];
  }

  return String(selectedValue)
    .split(',')
    .map((value) => value.trim())
    .filter(Boolean);
}

function isSelectedOption(value, selectedValue) {
  const values = Array.isArray(selectedValue) ? selectedValue : selectedValueList(selectedValue);
  return values.includes(String(value ?? ''));
}

function optionItems(options, placeholder, multiple = false) {
  const items = options.map((option) => ({ id: String(option.id), name: option.name }));

  return multiple ? items : [{ id: '', name: placeholder }, ...items];
}

function selectedValuesForOptions(options, selectedValue, multiple = false) {
  const optionValues = new Set(options.map((option) => String(option.id)));
  const values = selectedValueList(selectedValue).filter((value) => optionValues.has(value));

  return multiple ? values : values[0] || '';
}

function optionMarkup(options, selectedValue, placeholder, multiple = false) {
  const values = selectedValuesForOptions(options, selectedValue, multiple);

  return optionItems(options, placeholder, multiple)
    .map(
      (option) =>
        `<option value="${escapeHtml(option.id)}" ${isSelectedOption(option.id, values) ? 'selected' : ''}>${escapeHtml(option.name)}</option>`,
    )
    .join('');
}

function dropdownOptionMarkup(items, selectedValue) {
  return items
    .map(
      (option) => `
        <button
          class="app-select__option ${isSelectedOption(option.id, selectedValue) ? 'is-selected' : ''}"
          type="button"
          role="option"
          aria-selected="${isSelectedOption(option.id, selectedValue)}"
          data-custom-select-option
          data-value="${escapeHtml(option.id)}"
        >
          ${escapeHtml(option.name)}
        </button>
      `,
    )
    .join('');
}

function selectedLabel(items, selectedValue, placeholder, multiple = false) {
  const values = selectedValueList(selectedValue);
  const selectedItems = items.filter((item) => values.includes(String(item.id)));

  if (!multiple) {
    return selectedItems[0]?.name || items[0]?.name || placeholder;
  }

  if (!selectedItems.length) {
    return placeholder;
  }

  if (selectedItems.length <= 2) {
    return selectedItems.map((item) => item.name).join(', ');
  }

  return `${selectedItems.length} selected`;
}

function shouldCloseMultiSelectAfterChoice() {
  return window.matchMedia('(hover: none), (pointer: coarse)').matches;
}

const MENU_EDGE_GAP = 8;
const MENU_TRIGGER_GAP = 7;
const MENU_MAX_HEIGHT = 280;
const MENU_MIN_HEIGHT = 96;
const MENU_MIN_WIDTH = 160;
const dropdownMenus = new WeakMap();
const menuAnchors = new WeakMap();
const menuOwners = new WeakMap();

function menuForDropdown(dropdown) {
  const menu = dropdownMenus.get(dropdown) || dropdown.querySelector('[data-custom-select-menu]');

  if (menu) {
    dropdownMenus.set(dropdown, menu);
    menuOwners.set(menu, dropdown);
  }

  return menu;
}

function optionElements(dropdown) {
  return Array.from(menuForDropdown(dropdown)?.querySelectorAll('[data-custom-select-option]') || []);
}

function floatDropdownMenu(dropdown) {
  const menu = menuForDropdown(dropdown);

  if (!menu) {
    return;
  }

  if (!menuAnchors.has(dropdown)) {
    const anchor = document.createComment('custom-select-menu');
    menu.parentNode?.insertBefore(anchor, menu);
    menuAnchors.set(dropdown, anchor);
  }

  if (menu.parentNode !== document.body) {
    document.body.append(menu);
  }
}

function restoreDropdownMenu(dropdown) {
  const menu = menuForDropdown(dropdown);
  const anchor = menuAnchors.get(dropdown);

  if (!menu || !anchor) {
    return;
  }

  if (anchor.parentNode) {
    anchor.parentNode.insertBefore(menu, anchor);
    anchor.remove();
  } else if (menu.parentNode === document.body) {
    menu.remove();
  }

  menuAnchors.delete(dropdown);
}

function positionDropdownMenu(dropdown) {
  const trigger = dropdown.querySelector('[data-custom-select-trigger]');
  const menu = menuForDropdown(dropdown);

  if (!trigger || !menu || menu.hidden) {
    return;
  }

  const triggerRect = trigger.getBoundingClientRect();
  const viewportWidth = document.documentElement.clientWidth || window.innerWidth;
  const viewportHeight = document.documentElement.clientHeight || window.innerHeight;
  const maxWidth = Math.max(MENU_MIN_HEIGHT, viewportWidth - MENU_EDGE_GAP * 2);
  const width = Math.min(Math.max(triggerRect.width, MENU_MIN_WIDTH), maxWidth);
  const left = Math.min(
    Math.max(MENU_EDGE_GAP, triggerRect.left),
    Math.max(MENU_EDGE_GAP, viewportWidth - width - MENU_EDGE_GAP),
  );
  const spaceBelow = viewportHeight - triggerRect.bottom - MENU_TRIGGER_GAP - MENU_EDGE_GAP;
  const spaceAbove = triggerRect.top - MENU_TRIGGER_GAP - MENU_EDGE_GAP;
  const openAbove = spaceBelow < MENU_MIN_HEIGHT && spaceAbove > spaceBelow;
  const availableSpace = Math.max(MENU_MIN_HEIGHT, openAbove ? spaceAbove : spaceBelow);
  const maxHeight = Math.min(MENU_MAX_HEIGHT, availableSpace);

  menu.style.left = `${left}px`;
  menu.style.right = 'auto';
  menu.style.top = '0px';
  menu.style.width = `${width}px`;
  menu.style.maxHeight = `${maxHeight}px`;

  const menuHeight = menu.getBoundingClientRect().height;
  const top = openAbove
    ? Math.max(MENU_EDGE_GAP, triggerRect.top - MENU_TRIGGER_GAP - menuHeight)
    : Math.min(
        triggerRect.bottom + MENU_TRIGGER_GAP,
        Math.max(MENU_EDGE_GAP, viewportHeight - MENU_EDGE_GAP - menuHeight),
      );

  menu.style.top = `${top}px`;
  menu.dataset.placement = openAbove ? 'top' : 'bottom';
}

export function renderDropdown({
  name,
  label,
  options,
  selectedValue,
  placeholder,
  disabled = false,
  labelClass = 'filter-step',
  multiple = false,
  hideLabel = false,
}) {
  const normalizedValue = selectedValuesForOptions(options, selectedValue, multiple);
  const items = optionItems(options, placeholder, multiple);
  const displayLabel = selectedLabel(items, normalizedValue, placeholder, multiple);
  const disabledAttribute = disabled ? 'disabled' : '';
  const multipleAttribute = multiple ? 'multiple' : '';
  const multiselectAttribute = multiple ? 'aria-multiselectable="true"' : '';

  return `
    <label class="${escapeHtml(labelClass)} ${disabled ? 'is-disabled' : ''}">
      <span class="${hideLabel ? 'sr-only' : ''}">${escapeHtml(label)}</span>
      <div
        class="app-select ${disabled ? 'is-disabled' : ''}"
        data-custom-select
        data-placeholder="${escapeHtml(placeholder)}"
      >
        <select class="app-select__native" name="${escapeHtml(name)}" aria-label="${escapeHtml(label)}" ${disabledAttribute} ${multipleAttribute}>
          ${optionMarkup(options, normalizedValue, placeholder, multiple)}
        </select>
        <button
          class="app-select__trigger"
          type="button"
          aria-haspopup="listbox"
          aria-expanded="false"
          data-custom-select-trigger
          ${disabledAttribute}
        >
          <span data-custom-select-label>${escapeHtml(displayLabel)}</span>
          <span class="app-select__arrow" aria-hidden="true"></span>
        </button>
        <div class="app-select__menu" role="listbox" data-custom-select-menu ${multiselectAttribute} hidden>
          ${dropdownOptionMarkup(items, normalizedValue)}
        </div>
      </div>
    </label>
  `;
}

function syncCustomDropdown(dropdown) {
  const select = dropdown.querySelector('select');
  const trigger = dropdown.querySelector('[data-custom-select-trigger]');
  const label = dropdown.querySelector('[data-custom-select-label]');
  const options = optionElements(dropdown);
  const selectedOptions = Array.from(select.selectedOptions);
  const selectedValues = selectedOptions.map((option) => option.value);
  const placeholder = dropdown.dataset.placeholder || '';
  const selectedText = selectedOptions.map((option) => option.textContent.trim()).filter(Boolean);

  if (select.multiple) {
    if (!selectedText.length) {
      label.textContent = placeholder;
    } else if (selectedText.length <= 2) {
      label.textContent = selectedText.join(', ');
    } else {
      label.textContent = `${selectedText.length} selected`;
    }
  } else {
    label.textContent = selectedText[0] || placeholder;
  }

  trigger.disabled = select.disabled;
  dropdown.classList.toggle('is-disabled', select.disabled);
  dropdown.closest('.filter-step')?.classList.toggle('is-disabled', select.disabled);
  dropdown.classList.toggle('has-selection', selectedValues.length > 0);

  options.forEach((option) => {
    const selected = selectedValues.includes(option.dataset.value);
    option.classList.toggle('is-selected', selected);
    option.setAttribute('aria-selected', String(selected));
  });
}

export function setDropdownOptions(select, { options, selectedValue = '', placeholder, disabled = false }) {
  const dropdown = select.closest('[data-custom-select]');
  const menu = menuForDropdown(dropdown);
  const normalizedValue = selectedValuesForOptions(options, selectedValue, select.multiple);

  dropdown.dataset.placeholder = placeholder;
  select.innerHTML = optionMarkup(options, normalizedValue, placeholder, select.multiple);
  select.disabled = disabled;
  menu.innerHTML = dropdownOptionMarkup(optionItems(options, placeholder, select.multiple), normalizedValue);

  syncCustomDropdown(dropdown);
}

export function initCustomDropdowns(root, signal) {
  const dropdowns = Array.from(root.querySelectorAll('[data-custom-select]'));

  const closeDropdown = (dropdown) => {
    const menu = menuForDropdown(dropdown);
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');

    if (!menu || !trigger) {
      return;
    }

    menu.hidden = true;
    trigger.setAttribute('aria-expanded', 'false');
    dropdown.classList.remove('is-open');
    restoreDropdownMenu(dropdown);
  };

  const closeAll = (except = null) => {
    dropdowns.forEach((dropdown) => {
      if (dropdown !== except) {
        closeDropdown(dropdown);
      }
    });
  };

  const focusOption = (dropdown, target = 'selected') => {
    const options = optionElements(dropdown);
    const selected = options.find((option) => option.classList.contains('is-selected'));

    if (target === 'last') {
      options.at(-1)?.focus();
      return;
    }

    (target === 'first' ? options[0] : selected || options[0])?.focus();
  };

  const positionOpenDropdowns = () => {
    dropdowns.forEach((dropdown) => {
      if (dropdown.classList.contains('is-open')) {
        positionDropdownMenu(dropdown);
      }
    });
  };

  const openDropdown = (dropdown, focusTarget = null) => {
    const select = dropdown.querySelector('select');
    const menu = menuForDropdown(dropdown);
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');

    if (select.disabled) {
      return;
    }

    closeAll(dropdown);
    floatDropdownMenu(dropdown);
    menu.hidden = false;
    trigger.setAttribute('aria-expanded', 'true');
    dropdown.classList.add('is-open');
    positionDropdownMenu(dropdown);

    if (focusTarget) {
      focusOption(dropdown, focusTarget);
    }
  };

  dropdowns.forEach((dropdown) => {
    const select = dropdown.querySelector('select');
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');
    const menu = menuForDropdown(dropdown);

    syncCustomDropdown(dropdown);

    trigger.addEventListener('click', () => {
      if (menu.hidden) {
        openDropdown(dropdown);
      } else {
        closeDropdown(dropdown);
      }
    });

    trigger.addEventListener('keydown', (event) => {
      if (!['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
        return;
      }

      event.preventDefault();
      openDropdown(dropdown, event.key === 'End' ? 'last' : 'selected');
    });

    select.addEventListener('change', () => {
      syncCustomDropdown(dropdown);
    });

    menu.addEventListener('click', (event) => {
      const option = event.target.closest('[data-custom-select-option]');

      if (!option) {
        return;
      }

      if (select.multiple) {
        const nativeOption = Array.from(select.options).find((item) => item.value === option.dataset.value);

        if (nativeOption) {
          nativeOption.selected = !nativeOption.selected;
        }

        select.dispatchEvent(new Event('change', { bubbles: true }));
        syncCustomDropdown(dropdown);

        if (shouldCloseMultiSelectAfterChoice()) {
          closeDropdown(dropdown);
          trigger.focus();
        } else {
          option.focus();
        }

        return;
      }

      select.value = option.dataset.value;
      select.dispatchEvent(new Event('change', { bubbles: true }));
      syncCustomDropdown(dropdown);
      closeDropdown(dropdown);
      trigger.focus();
    });

    menu.addEventListener('keydown', (event) => {
      const option = event.target.closest('[data-custom-select-option]');

      if (event.key === 'Escape') {
        event.preventDefault();
        closeDropdown(dropdown);
        trigger.focus();
        return;
      }

      if (!option) {
        return;
      }

      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        option.click();
        return;
      }

      if (!['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
        return;
      }

      event.preventDefault();
      const options = optionElements(dropdown);
      const currentIndex = options.indexOf(option);
      let nextIndex = currentIndex;

      if (event.key === 'ArrowDown') nextIndex = currentIndex + 1;
      if (event.key === 'ArrowUp') nextIndex = currentIndex - 1;
      if (event.key === 'Home') nextIndex = 0;
      if (event.key === 'End') nextIndex = options.length - 1;

      options[Math.max(0, Math.min(nextIndex, options.length - 1))]?.focus();
    });
  });

  document.addEventListener(
    'click',
    (event) => {
      const menu = event.target.closest('[data-custom-select-menu]');

      if (!event.target.closest('[data-custom-select]') && !(menu && menuOwners.has(menu))) {
        closeAll();
      }
    },
    { signal },
  );

  document.addEventListener(
    'keydown',
    (event) => {
      if (event.key === 'Escape') {
        closeAll();
      }
    },
    { signal },
  );

  window.addEventListener('resize', positionOpenDropdowns, { signal });
  document.addEventListener('scroll', positionOpenDropdowns, { signal, capture: true });
  signal?.addEventListener('abort', () => dropdowns.forEach((dropdown) => closeDropdown(dropdown)), {
    once: true,
  });
}
