import type { Issue } from './issue';

export interface IssuePage {
  items: Issue[];
  page: number;
  page_size: number;
  total: number;
}
