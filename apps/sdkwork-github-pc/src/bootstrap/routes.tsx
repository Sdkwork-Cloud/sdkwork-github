import { BrowserRouter, Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { SdkworkIamAuthRoutes, SdkworkSessionAuthBrowserRoot } from '@sdkwork/auth-pc-react';
import { GithubAuthGate } from '@sdkwork/github-pc-core';
import { AppShell } from '@sdkwork/github-pc-shell';
import type { GithubPcRuntime } from '@sdkwork/github-pc-core';

function AuthRoutes({ runtime }: { runtime: GithubPcRuntime }) {
  return (
    <SdkworkIamAuthRoutes
      basePath="/auth"
      getRuntime={() => runtime.iamRuntime as never}
      runtimeConfig={{
        loginMethods: ['password'],
        qrLoginEnabled: true,
        registerMethods: ['email', 'phone'],
        verificationPolicy: {
          emailCodeLoginEnabled: false,
          phoneCodeLoginEnabled: false,
        },
      }}
    />
  );
}

function RoutedAuthGate({ runtime }: { runtime: GithubPcRuntime }) {
  const location = useLocation();
  const navigate = useNavigate();
  return (
    <GithubAuthGate
      authRoutes={<AuthRoutes runtime={runtime} />}
      location={location}
      navigate={(to, options) => navigate(to, options)}
      session={runtime.session}
    >
      <AppShell />
    </GithubAuthGate>
  );
}

export function GithubAppRoutes({ runtime }: { runtime: GithubPcRuntime }) {
  return (
    <BrowserRouter>
      <SdkworkSessionAuthBrowserRoot>
        <Routes>
          <Route path="/*" element={<RoutedAuthGate runtime={runtime} />} />
        </Routes>
      </SdkworkSessionAuthBrowserRoot>
    </BrowserRouter>
  );
}
