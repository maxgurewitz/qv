import React from 'react';
import { BrowserRouter as Router, Route } from "react-router-dom";
import './App.css';
import Home from './Home';
import CssBaseline from '@material-ui/core/CssBaseline';

const App: React.FC = () => {
  return (
    <div className="App">
      <CssBaseline />
      <Router>
        <Route exact path="/" component={Home} />
      </Router>
    </div>
  );
}

export default App;
