import * as libWidgets from './manifest';

export default function renderWidget(name: string) {
  return libWidgets[name];
}
