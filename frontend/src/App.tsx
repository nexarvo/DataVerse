import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css';
import './index.css';
import LandingPage from './pages/LandingPage';
import AuthPage from './pages/AuthPage';
import Dashboard from './pages/Dashboard';
import TransformationsPage from './pages/TransformationPage';
import DataPage from './pages/DatasetsPage';

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <Routes>
        {/* Public Routes */}
        <Route path='/' element={<LandingPage />} />
        <Route path='/signup' element={<AuthPage />} />

        {/* Dashboard (Private Layout) */}
        <Route path='/dashboard' element={<Dashboard />}>
          {/* Nested Routes for Dashboard */}
          <Route path='data' element={<DataPage />} />
          <Route path='transformations' element={<TransformationsPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default App;
