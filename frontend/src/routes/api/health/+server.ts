import { json } from '@sveltejs/kit';

export function GET() {
  return json({ status: 'ok' }, { status: 200 });
}
