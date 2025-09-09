export function getAppId(): string {
  return import.meta.env.VITE_APP_ID || 'placeholder';
}

function toTitleCase(id: string): string {
  // Convert things like "my-app_name" -> "My App Name"
  const parts = id
    .replace(/([a-z0-9])([A-Z])/g, '$1 $2') // split camelCase: myApp -> my App
    .split(/[-_\s]+/)
    .filter(Boolean);
  return parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1).toLowerCase()).join(' ');
}

export function getAppTitle(): string {
  return toTitleCase(getAppId());
}
