import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css';
import './index.css';
import LandingPage from './pages/LandingPage';
import AuthPage from './pages/AuthPage';
import Dashboard from './pages/Dashboard';
import TransformationsPage from './pages/TransformationPage';

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path='/' element={<LandingPage />} />
        <Route path='/signup' element={<AuthPage />} />
        <Route path='/dashboard' element={<Dashboard />} />
        <Route path='/transformations' element={<TransformationsPage />} />
      </Routes>
    </BrowserRouter>
  );
};

export default App;
