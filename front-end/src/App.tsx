// src/App.tsx
import React from 'react';
import MainLayout from './layouts/MainLayout';
import HomePage from './pages/HomePage';

const App: React.FC = () => {
  return (
    <MainLayout>
      <HomePage />
    </MainLayout>
  );
};

export default App;
