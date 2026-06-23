import type { Plan } from './plan';

export interface PlanPage {
  items: Plan[];
  page: number;
  page_size: number;
  total: number;
}
