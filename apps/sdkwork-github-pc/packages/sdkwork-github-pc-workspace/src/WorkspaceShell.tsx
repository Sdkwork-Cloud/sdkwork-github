import { NavLink, Outlet } from 'react-router-dom';
import { APP_TITLE } from '@sdkwork/github-pc-core';

const linkStyle = ({ isActive }: { isActive: boolean }) => ({
  fontWeight: isActive ? 700 : 400,
  marginRight: '1rem',
});

export function WorkspaceShell() {
  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '1.5rem' }}>
      <header style={{ marginBottom: '1.5rem' }}>
        <h1 style={{ margin: '0 0 0.5rem' }}>{APP_TITLE}</h1>
        <nav aria-label="GitHub workspace">
          <NavLink to="/repositories" style={linkStyle}>Repositories</NavLink>
          <NavLink to="/issues" style={linkStyle}>Issues</NavLink>
          <NavLink to="/plans" style={linkStyle}>Plans</NavLink>
        </nav>
      </header>
      <main>
        <Outlet />
      </main>
    </div>
  );
}
