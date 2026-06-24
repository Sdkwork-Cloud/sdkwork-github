import type { PlanItem } from './plan-item';

export interface Plan {
  id: string;
  title: string;
  status: string;
  repository_id?: string;
  items: PlanItem[];
}
