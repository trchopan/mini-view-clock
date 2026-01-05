<script lang="ts">
    import {currentView} from './store'
    import {View} from './types'

    let syncId = ''
    let error: string | null = null

    const SYNC_ID_RE = /^[0-9]{8}$/

    function normalizeId(v: string) {
        return v.trim()
    }

    function go() {
        const id = normalizeId(syncId)
        if (!SYNC_ID_RE.test(id)) {
            error = 'Please enter a valid UUID (e.g., 123e4567-e89b-12d3-a456-426614174000).'
            return
        }
        error = null

        // Navigate to synced route
        window.location.assign(`/sync/${id}`)
    }

    function backToClock() {
        currentView.set(View.Clock)
    }
</script>

<div class="h-screen w-screen flex items-center justify-center text-white p-6">
    <div class="w-full max-w-lg bg-gray-900 rounded-lg p-6 border border-gray-800">
        <div class="flex items-center justify-between mb-4">
            <h1 class="text-lg font-semibold">Join a Sync Room</h1>
            <button class="btn" on:click={backToClock} type="button">Back</button>
        </div>

        <label class="text-sm text-gray-300 block mb-2" for="syncId">Sync ID (UUID)</label>

        <div class="flex gap-2">
            <input
                id="syncId"
                class="input flex-1"
                placeholder="8-digit sync code"
                bind:value={syncId}
                inputmode="numeric"
                pattern="[0-9]*"
                maxlength="8"
                on:input={() => {
                    syncId = syncId.replace(/\D/g, '').slice(0, 8)
                }}
                on:keydown={e => {
                    if (e.key === 'Enter') go()
                }}
            />

            <button class="btn-primary" on:click={go} type="button">Go</button>
        </div>

        {#if error}
            <div class="mt-3 text-sm text-red-400">{error}</div>
        {/if}

        <div class="mt-4 text-xs text-gray-400">
            This will open <span class="text-gray-200">/sync/&lt;id&gt;</span> and connect to the sync
            server.
        </div>
    </div>
</div>

<style>
    .input {
        @apply px-3 py-2 rounded bg-gray-800 text-white border border-gray-700 outline-none;
    }
    .input:focus {
        @apply border-gray-500;
    }
    .btn {
        @apply px-3 py-2 rounded bg-gray-800 text-white text-sm border border-gray-700;
    }
    .btn-primary {
        @apply px-3 py-2 rounded bg-blue-600 text-white text-sm;
    }
</style>
