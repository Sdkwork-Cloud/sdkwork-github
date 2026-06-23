export interface SessionSnapshot {
  authToken?: string;
  accessToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: {
    tenantId?: string;
    userId?: string;
    organizationId?: string;
    sessionId?: string;
  };
}

export interface SessionStore {
  getSnapshot(): SessionSnapshot;
  setSession(nextSession: SessionSnapshot): void;
  clearSession(): void;
  subscribe(listener: (snapshot: SessionSnapshot) => void): () => void;
}

export interface SessionStorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

const DEFAULT_SESSION_STORAGE_KEY = 'sdkwork-github-pc-session';

function readInitialSession(
  storage: SessionStorageLike | undefined,
  storageKey: string,
): SessionSnapshot {
  if (!storage) return {};
  try {
    const raw = storage.getItem(storageKey);
    return raw ? (JSON.parse(raw) as SessionSnapshot) : {};
  } catch {
    return {};
  }
}

export function createSessionStore(
  storage?: SessionStorageLike,
  storageKey = DEFAULT_SESSION_STORAGE_KEY,
): SessionStore {
  let snapshot = readInitialSession(storage, storageKey);
  const listeners = new Set<(snapshot: SessionSnapshot) => void>();

  const emit = () => {
    for (const listener of listeners) {
      listener(snapshot);
    }
  };

  const persist = () => {
    if (!storage) return;
    if (!snapshot.authToken && !snapshot.accessToken && !snapshot.refreshToken) {
      storage.removeItem(storageKey);
      return;
    }
    storage.setItem(storageKey, JSON.stringify(snapshot));
  };

  return {
    getSnapshot: () => snapshot,
    setSession(nextSession) {
      snapshot = nextSession;
      persist();
      emit();
    },
    clearSession() {
      snapshot = {};
      persist();
      emit();
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

export function hasGithubIamSession(session: SessionSnapshot): boolean {
  return Boolean(session.authToken && session.accessToken && session.context?.tenantId);
}
