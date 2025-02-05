// src/pages/BlockPage.tsx
import React, { useState } from 'react';
import { fetchBlock } from '../services/api';

const BlockPage: React.FC = () => {
  const [blockNumber, setBlockNumber] = useState<string>('');
  const [blockData, setBlockData] = useState<any>(null);
  const [error, setError] = useState<string>('');

  const handleFetchBlock = async () => {
    try {
      setError('');
      setBlockData(null);
      const data = await fetchBlock(Number(blockNumber));
      setBlockData(data);
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div>
      <h2>Buscar Bloco</h2>
      <input
        type="text"
        placeholder="Número do bloco"
        value={blockNumber}
        onChange={(e) => setBlockNumber(e.target.value)}
      />
      <button onClick={handleFetchBlock}>Buscar</button>
      {error && <p style={{ color: 'red' }}>{error}</p>}
      {blockData && (
        <div style={{ marginTop: '1rem' }}>
          <h3>Dados do Bloco:</h3>
          <pre>{JSON.stringify(blockData, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default BlockPage;
