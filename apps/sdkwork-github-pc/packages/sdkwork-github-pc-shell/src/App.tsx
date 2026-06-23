import { Navigate, Route, Routes } from 'react-router-dom';
import {
  IssuesPage,
  PlansPage,
  RepositoriesPage,
  WorkspaceShell,
} from '@sdkwork/github-pc-workspace';

export function AppShell() {
  return (
    <Routes>
      <Route element={<WorkspaceShell />}>
        <Route index element={<Navigate replace to="/repositories" />} />
        <Route path="repositories" element={<RepositoriesPage />} />
        <Route path="issues" element={<IssuesPage />} />
        <Route path="plans" element={<PlansPage />} />
      </Route>
    </Routes>
  );
}
