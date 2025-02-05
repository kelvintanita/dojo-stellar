// src/pages/HomePage.tsx

import React, { useState } from 'react';
import BlockPage from './BlockPage';
import TransactionPage from './TransactionPage';
import BalancePage from './BalancePage';

type TabKey = 'block' | 'transaction' | 'balance';

const HomePage: React.FC = () => {
  // Estado para controlar qual aba está ativa
  const [activeTab, setActiveTab] = useState<TabKey>('block');

  // Função para trocar a aba ativa
  const handleTabChange = (tab: TabKey) => {
    setActiveTab(tab);
  };

  return (
    <div style={{ color: '#ccc' }}>
      {/* Container das "abas" */}
      <div style={{ display: 'flex', gap: '1rem', marginBottom: '1rem' }}>
        {/* Cada botão troca a aba para um componente diferente */}
        <button
          onClick={() => handleTabChange('block')}
          style={{
            backgroundColor: activeTab === 'block' ? '#444' : 'black',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
          }}
        >
          Blocos
        </button>
        <button
          onClick={() => handleTabChange('transaction')}
          style={{
            backgroundColor: activeTab === 'transaction' ? '#444' : 'black',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
          }}
        >
          Transações
        </button>
        <button
          onClick={() => handleTabChange('balance')}
          style={{
            backgroundColor: activeTab === 'balance' ? '#444' : 'black',
            color: '#fff',
            padding: '0.5rem 1rem',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
          }}
        >
          Saldo
        </button>
      </div>

      {/* Renderização condicional do conteúdo de cada aba */}
      <div>
        {activeTab === 'block' && (
          <div>
            <BlockPage />
          </div>
        )}

        {activeTab === 'transaction' && (
          <div>
            <TransactionPage />
          </div>
        )}

        {activeTab === 'balance' && (
          <div>
            <BalancePage />
          </div>
        )}
      </div>
    </div>
  );
};

export default HomePage;
