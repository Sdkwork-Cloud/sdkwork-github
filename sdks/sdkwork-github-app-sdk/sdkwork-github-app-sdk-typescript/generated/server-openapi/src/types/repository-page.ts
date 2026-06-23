import type { Repository } from './repository';

export interface RepositoryPage {
  items: Repository[];
  page: number;
  page_size: number;
  total: number;
}
