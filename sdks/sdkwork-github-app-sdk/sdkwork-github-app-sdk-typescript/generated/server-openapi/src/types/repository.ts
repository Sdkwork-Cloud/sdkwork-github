export interface Repository {
  id: string;
  full_name: string;
  owner: string;
  description?: string;
  default_branch?: string;
  html_url?: string;
  is_private: boolean;
}
