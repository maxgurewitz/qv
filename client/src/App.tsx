import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import './App.css';
import Home from './Home';
import { Route, Switch } from 'react-router' // react-router v4/v5
import { ConnectedRouter } from 'connected-react-router'
import { history } from './configureStore'

const App: React.FC = () => {
  return (
    <div className="App">
      <CssBaseline />
      <ConnectedRouter history={history}>
        <Switch>
          <Route exact path="/" component={Home} />
          <Route render={() => (<div>404</div>)} />
        </Switch>
      </ConnectedRouter>
    </div>
  );
}

export default App;
