// src/pages/HomePage.tsx
import React from 'react';
import { Link } from 'react-router-dom';

const HomePage: React.FC = () => {
  return (
    <div>
      <h2>Bem vindo ao meu buscador</h2>
      <p>Escolha uma opção:</p>
      <ul>
        <li><Link to="/block">Buscar Bloco</Link></li>
        <li><Link to="/transaction">Buscar Transação</Link></li>
        <li><Link to="/balance">Buscar Saldo</Link></li>
      </ul>
    </div>
  );
};

export default HomePage;
