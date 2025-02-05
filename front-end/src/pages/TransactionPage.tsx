import React, { useState } from 'react';
import { fetchTransaction } from '../services/api';

interface TransactionData {
  id: string;
  paging_token: string;
  successful: boolean;
  hash: string;
  ledger: number;
  created_at: string;
}

const TransactionPage: React.FC = () => {
  const [txHash, setTxHash] = useState<string>('');
  const [txData, setTxData] = useState<TransactionData | null>(null);
  const [error, setError] = useState<string>('');

  const handleFetchTransaction = async () => {
    try {
      setError('');
      setTxData(null);
      const data = await fetchTransaction(txHash);
      setTxData(data);
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleClear = () => {
    setTxHash('');
    setTxData(null);
    setError('');
  };

  return (
    <div style={{ color: '#ccc' }}>
      <h2 style={{ color: '#fff' }}>Buscar Transação</h2>

      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <input
          type="text"
          placeholder="Hash da transação"
          value={txHash}
          onChange={(e) => setTxHash(e.target.value)}
          style={{
            width: '600px',
            height: '2rem',
            borderRadius: '8px',
            backgroundColor: '#fff',
            color: '#333',
            padding: '0.5rem',
            border: '1px solid #999',
          }}
        />
        <button
          onClick={handleFetchTransaction}
          style={{
            backgroundColor: 'black',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '8px',
            cursor: 'pointer',
          }}
        >
          Buscar
        </button>

        <button
          onClick={handleClear}
          style={{
            backgroundColor: '#555',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '8px',
            cursor: 'pointer',
          }}
        >
          Limpar
        </button>
      </div>

      {error && <p style={{ color: 'red', marginTop: '1rem' }}>{error}</p>}

      {txData && (
        <div style={{ marginTop: '1rem' }}>
          <h3 style={{ color: '#fff' }}>Resultado da Transação:</h3>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            <div>
              <strong>ID:</strong> {txData.id}
            </div>
            <div>
              <strong>Paging Token:</strong> {txData.paging_token}
            </div>
            <div>
              <strong>Successful:</strong> {txData.successful ? 'Sim' : 'Não'}
            </div>
            <div>
              <strong>Hash:</strong> {txData.hash}
            </div>
            <div>
              <strong>Ledger:</strong> {txData.ledger}
            </div>
            <div>
              <strong>Criado em:</strong> {txData.created_at}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default TransactionPage;
