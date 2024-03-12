'use client'
import { emit } from '@tauri-apps/api/event'

export default function Home() {

  async function ping() {
    await emit('ping', { any: 'one' })
  }

  return (
    <div className="flex items-center justify-center w-screen h-screen">
      <button className="btn" onClick={() => ping()}>
        ping
      </button>
    </div>

  );
}
