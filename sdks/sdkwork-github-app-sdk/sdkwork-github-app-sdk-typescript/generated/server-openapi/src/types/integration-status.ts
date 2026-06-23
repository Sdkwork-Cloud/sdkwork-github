export interface IntegrationStatus {
  provider: string;
  linked: boolean;
  status?: string;
  external_account_id?: string;
  scopes?: string;
  last_synced_at?: string;
}
