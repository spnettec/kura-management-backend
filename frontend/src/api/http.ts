export async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const token = localStorage.getItem('kura-mgmt-auth')
    ? (JSON.parse(localStorage.getItem('kura-mgmt-auth') as string).token as string)
    : ''

  const response = await fetch(path, {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(init.headers || {}),
    },
  })

  if (response.status === 401 && !path.startsWith('/api/auth/login')) {
    localStorage.removeItem('kura-mgmt-auth')
    if (window.location.pathname !== '/login') {
      window.location.assign(`/login?redirect=${encodeURIComponent(window.location.pathname)}`)
    }
    throw new Error('Unauthorized')
  }

  if (!response.ok) {
    throw new Error(`Request failed: ${response.status}`)
  }

  if (response.status === 204) {
    return undefined as T
  }

  return (await response.json()) as T
}
