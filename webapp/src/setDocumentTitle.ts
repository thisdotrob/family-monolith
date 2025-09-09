import { getAppTitle } from './appMeta';

export function setDocumentTitle() {
  if (typeof document !== 'undefined') {
    document.title = getAppTitle();
  }
}
