import React, { useState } from 'react';
import { fetchBalance } from '../services/api';

const BalancePage: React.FC = () => {
  const [address, setAddress] = useState<string>('');
  const [saldo, setSaldo] = useState<number | null>(null);
  const [error, setError] = useState<string>('');

  const handleFetchBalance = async () => {
    try {
      setError('');
      setSaldo(null);
      const data = await fetchBalance(address);
      // Ajuste conforme a estrutura do retorno. Exemplo:
      setSaldo(data.balances[0].balance);
    } catch (err: any) {
      setError(err.message);
    }
  };

  // Função para limpar o input e o resultado
  const handleClear = () => {
    setAddress('');
    setSaldo(null);
    setError('');
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
          onClick={handleFetchBalance}
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

        {/* Botão "Limpar" */}
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

      {saldo !== null && (
        <div style={{ marginTop: '1rem' }}>
          <h3 style={{ color: '#fff' }}>Saldo: {saldo}</h3>
        </div>
      )}
    </div>
  );
};

export default BalancePage;
