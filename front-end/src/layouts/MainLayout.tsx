import React from 'react';

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        height: '100%',      
        backgroundColor: '#333',
      }}
    >
      <header
        style={{
          width: '100%',
          height: '160px',
          backgroundColor: 'black',
          color: 'white',
          padding: '1rem',
        }}
      >
        <h1 style={{ margin: 0 }}>Stellar Explorador de Blocos</h1>
      </header>

      <main
        style={{
          flex: 1,            // faz o "main" crescer e preencher o espaço restante
          padding: '1rem',
          color: '#ccc',
        }}
      >
        {children}
      </main>

      <footer
        style={{
          width: '100%',
          backgroundColor: 'black',
          color: 'white',
          textAlign: 'center',
          padding: '0.5rem 0',
        }}
      >
        <p style={{ margin: 0 }}>© 2025 - Explorador de Blocos</p>
      </footer>
    </div>
  );
};

export default MainLayout;
