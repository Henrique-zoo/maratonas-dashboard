import * as competitionPage from '../pages/competition.js';
import * as competitionsPage from '../pages/competitions.js';
import * as eventPage from '../pages/event.js';
import * as homePage from '../pages/home.js';
import * as institutionPage from '../pages/institution.js';
import * as institutionsPage from '../pages/institutions.js';
import * as organizerPage from '../pages/organizer.js';
import * as organizersPage from '../pages/organizers.js';
import * as teamPage from '../pages/team.js';
import * as teamsPage from '../pages/teams.js';

const routes = [
  { pattern: /^\/$/, view: homePage.render, navKey: '/' },
  { pattern: /^\/competitions$/, view: competitionsPage.render, navKey: '/competitions' },
  { pattern: /^\/competitions\/(?<id>\d+)$/, view: competitionPage.render, navKey: '/competitions' },
  { pattern: /^\/teams$/, view: teamsPage.render, navKey: '/teams' },
  { pattern: /^\/teams\/(?<id>\d+)$/, view: teamPage.render, navKey: '/teams' },
  { pattern: /^\/institutions$/, view: institutionsPage.render, navKey: '/institutions' },
  { pattern: /^\/institutions\/(?<id>\d+)$/, view: institutionPage.render, navKey: '/institutions' },
  { pattern: /^\/organizers$/, view: organizersPage.render, navKey: '/organizers' },
  { pattern: /^\/organizers\/(?<id>\d+)$/, view: organizerPage.render, navKey: '/organizers' },
  { pattern: /^\/events\/(?<id>\d+)$/, view: eventPage.render, navKey: null },
];

export function matchRoute(pathname) {
  for (const route of routes) {
    const match = pathname.match(route.pattern);

    if (match) {
      return {
        ...route,
        params: match.groups || {},
      };
    }
  }

  return null;
}
