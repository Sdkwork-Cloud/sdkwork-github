export interface Issue {
  id: string;
  repository_id: string;
  number: number;
  title: string;
  state: string;
  html_url?: string;
}
