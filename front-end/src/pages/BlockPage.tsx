// src/pages/BlockPage.tsx
import React, { useState } from 'react';
import { fetchBlock } from '../services/api';

interface BlockData {
  id: string;
  paging_token: string;
  hash: string;
  prev_hash: string;
  sequence: number;
  successful_transaction_count: number;
  failed_transaction_count: number;
  operation_count: number;
  tx_set_operation_count: number;
  closed_at: string;
  total_coins: string;
  fee_pool: string;
  base_fee_in_stroops: number;
  base_reserve_in_stroops: number;
  max_tx_set_size: number;
  protocol_version: number;
  header_xdr: string;
}

const BlockPage: React.FC = () => {
  const [blockNumber, setBlockNumber] = useState<string>('');
  const [blockData, setBlockData] = useState<BlockData | null>(null);
  const [error, setError] = useState<string>('');

  const handleFetchBlock = async () => {
    try {
      setError('');
      setBlockData(null);
      const data = await fetchBlock(Number(blockNumber));

      // Ajuste se o JSON retornado tiver nomes/propriedades diferentes
      setBlockData(data);
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div style={{ color: '#ccc' }}>
      <h2 style={{ color: '#fff' }}>Buscar Bloco</h2>

      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <input
          type="text"
          placeholder="Número do bloco"
          value={blockNumber}
          onChange={(e) => setBlockNumber(e.target.value)}
          style={{
            width: '300px',
            height: '2rem',
            borderRadius: '8px',
            backgroundColor: '#fff',
            color: '#333',
            padding: '0.5rem',
            border: '1px solid #999',
          }}
        />
        <button
          onClick={handleFetchBlock}
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
      </div>

      {error && <p style={{ color: 'red', marginTop: '1rem' }}>{error}</p>}

      {blockData && (
        <div style={{ marginTop: '1rem' }}>
          <h3 style={{ color: '#fff' }}>Dados do Bloco:</h3>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            <div>
              <strong>ID:</strong> {blockData.id}
            </div>
            <div>
              <strong>Paging Token:</strong> {blockData.paging_token}
            </div>
            <div>
              <strong>Hash:</strong> {blockData.hash}
            </div>
            <div>
              <strong>Prev Hash:</strong> {blockData.prev_hash}
            </div>
            <div>
              <strong>Sequence:</strong> {blockData.sequence}
            </div>
            <div>
              <strong>Successful TX Count:</strong>{' '}
              {blockData.successful_transaction_count}
            </div>
            <div>
              <strong>Failed TX Count:</strong> {blockData.failed_transaction_count}
            </div>
            <div>
              <strong>Operation Count:</strong> {blockData.operation_count}
            </div>
            <div>
              <strong>TX Set Operation Count:</strong>{' '}
              {blockData.tx_set_operation_count}
            </div>
            <div>
              <strong>Closed At:</strong> {blockData.closed_at}
            </div>
            <div>
              <strong>Total Coins:</strong> {blockData.total_coins}
            </div>
            <div>
              <strong>Fee Pool:</strong> {blockData.fee_pool}
            </div>
            <div>
              <strong>Base Fee (stroops):</strong> {blockData.base_fee_in_stroops}
            </div>
            <div>
              <strong>Base Reserve (stroops):</strong>{' '}
              {blockData.base_reserve_in_stroops}
            </div>
            <div>
              <strong>Max TX Set Size:</strong> {blockData.max_tx_set_size}
            </div>
            <div>
              <strong>Protocol Version:</strong> {blockData.protocol_version}
            </div>
            <div>
              <strong>Header XDR:</strong> {blockData.header_xdr}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default BlockPage;
