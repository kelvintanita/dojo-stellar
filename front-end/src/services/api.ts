// src/services/api.ts
const BASE_URL = 'https://dojo-stellar-11822830061.us-central1.run.app';

export async function fetchBlock(blockNumber: number) {
  const response = await fetch(`${BASE_URL}/block/${blockNumber}`);
  if (!response.ok) {
    throw new Error(`Erro ao buscar bloco ${blockNumber}`);
  }
  return response.json();
}

export async function fetchTransaction(txHash: string) {
  const response = await fetch(`${BASE_URL}/transaction/${txHash}`);
  if (!response.ok) {
    throw new Error(`Erro ao buscar transação ${txHash}`);
  }
  return response.json();
}

export async function fetchBalance(address: string) {
  const response = await fetch(`${BASE_URL}/balance/${address}`);
  if (!response.ok) {
    throw new Error(`Erro ao buscar saldo do endereço ${address}`);
  }
  return response.json();
}
