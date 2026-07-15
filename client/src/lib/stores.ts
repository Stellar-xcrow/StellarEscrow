import { writable } from 'svelte/store';
import { env } from '$lib/env';

// contractId: set this to your deployed Soroban contract address before going live.
// Example: 'CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ'
export const contractId = writable('CDVOID__...'); // Replace with deployed contract ID
export const network = writable<'testnet' | 'mainnet'>(env.network);
