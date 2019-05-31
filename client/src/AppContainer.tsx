import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import { connect } from 'react-redux';
import { Route, Switch, Redirect } from 'react-router';
import { ConnectedRouter } from 'connected-react-router';
import { history } from './configureStore';
import './App.css';
import Home from './HomeContainer';
import NotFound from './NotFound';
import { CombinedState } from './types';

const LoggedIn: React.FC = () => {
  return (<div> logged in </div>);
};

interface AppProps {
  loggedIn: boolean
}

const App: React.FC<AppProps> = ({ loggedIn }) => {
  return (
    <div className="App">
      <CssBaseline />
      <ConnectedRouter history={history}>
        <Switch>
          <Route exact path="/" render={() => (
            loggedIn ?
              <Redirect to="/app" /> :
              <Home />
          )} />
          <Route path="/app" render={() => (
            loggedIn ?
              <LoggedIn /> :
              <Redirect to="/" />
          )} />
          <Route component={NotFound} />
        </Switch>
      </ConnectedRouter>
    </div>
  );
};

function mapStateToProps(state: CombinedState): AppProps {
  return {
    loggedIn: state.primary.accessToken !== null
  };
};

export default connect(mapStateToProps)(App);
