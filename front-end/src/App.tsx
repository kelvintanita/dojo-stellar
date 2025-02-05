import React from 'react';
import { Route, Routes } from 'react-router-dom';
import MainLayout from './layouts/MainLayout';
import HomePage from './pages/HomePage';
import BlockPage from './pages/BlockPage';
import TransactionPage from './pages/TransactionPage';
import BalancePage from './pages/BalancePage';

const App: React.FC = () => {
  return (
    <MainLayout>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/block" element={<BlockPage />} />
        <Route path="/transaction" element={<TransactionPage />} />
        <Route path="/balance" element={<BalancePage />} />
      </Routes>
    </MainLayout>
  );
};

export default App;
