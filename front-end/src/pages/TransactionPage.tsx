// src/pages/TransactionPage.tsx
import React, { useState } from 'react';
import { fetchTransaction } from '../services/api';

const TransactionPage: React.FC = () => {
  const [txHash, setTxHash] = useState<string>('');
  const [txData, setTxData] = useState<any>(null);
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

  return (
    <div>
      <h2>Buscar Transação</h2>
      <input
        type="text"
        placeholder="Hash da transação"
        value={txHash}
        onChange={(e) => setTxHash(e.target.value)}
      />
      <button onClick={handleFetchTransaction}>Buscar</button>
      {error && <p style={{ color: 'red' }}>{error}</p>}
      {txData && (
        <div style={{ marginTop: '1rem' }}>
          <h3>Dados da Transação:</h3>
          <pre>{JSON.stringify(txData, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default TransactionPage;
