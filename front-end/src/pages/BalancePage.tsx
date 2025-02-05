// src/pages/BalancePage.tsx
import React, { useState } from 'react';
import { fetchBalance } from '../services/api';

const BalancePage: React.FC = () => {
  const [address, setAddress] = useState<string>('');
  const [balanceData, setBalanceData] = useState<any>(null);
  const [error, setError] = useState<string>('');

  const handleFetchBalance = async () => {
    try {
      setError('');
      setBalanceData(null);
      const data = await fetchBalance(address);
      setBalanceData(data);
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div style={{ color: '#ccc' }}>
      <h2 style={{ color: '#fff' }}>Buscar Saldo por Endereço</h2>
      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <input
          type="text"
          placeholder="Endereço"
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          style={{
            width: '600px',        // Largura maior para facilitar a leitura
            height: '2rem',        // Altura razoável
            borderRadius: '8px',   // Cantos arredondados
            backgroundColor: '#fff', 
            color: '#333',
            padding: '0.5rem',     // Espaçamento interno
            border: '1px solid #999',
          }}
        />
        <button
          onClick={handleFetchBalance}
          style={{
            backgroundColor: 'black',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '8px', // Cantos arredondados do botão
            cursor: 'pointer',
          }}
        >
          Buscar
        </button>
      </div>

      {error && <p style={{ color: 'red', marginTop: '1rem' }}>{error}</p>}
      {balanceData && (
        <div style={{ marginTop: '1rem' }}>
          <h3 style={{ color: '#fff' }}>Saldo:</h3>
          <pre style={{ color: '#fff', backgroundColor: '#444', padding: '1rem' }}>
            {JSON.stringify(balanceData, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
};

export default BalancePage;
