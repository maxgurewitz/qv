import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import { connect } from 'react-redux';
import { Route, Switch, Redirect } from 'react-router';
import { ConnectedRouter } from 'connected-react-router';
import AppBar from '@material-ui/core/AppBar';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';
import { history } from './configureStore';
import './App.css';
import Home from './HomeContainer';
import AuthCallback from './AuthCallbackContainer';
import Initializing from './InitializingContainer';
import NotFound from './NotFound';
import { CombinedState } from './types';

const LoggedIn: React.FC = () => {
  return (
    <AppBar position="static">
      <Tabs value={0} onChange={console.log}>
        <Tab label="Item One" />
        <Tab label="Item Two" />
        <Tab label="Item Three" />
      </Tabs>
      <div>
        Logged in
      </div>
    </AppBar>
  );
};

interface InitializedProps {
  loggedIn: boolean,
}

const Initialized: React.FC<InitializedProps> = ({ loggedIn }) => {
  return (
    <React.Fragment>
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
      <Route exact path="/auth-callback" render={() => (
        loggedIn ?
          <Redirect to="/app" /> :
          <AuthCallback />
      )} />
      <Route component={NotFound} />
    </React.Fragment>
  );
}

interface AppProps {
  loggedIn: boolean,
  initializing: boolean
}

const App: React.FC<AppProps> = ({ loggedIn, initializing }) => {
  return (
    <div className="App">
      <CssBaseline />
      <ConnectedRouter history={history}>
        <Switch>
          <Route render={() => (
            initializing ?
              <Initializing/> :
              <Initialized loggedIn={loggedIn} />
          )} />
        </Switch>
      </ConnectedRouter>
    </div>
  );
};

function mapStateToProps(state: CombinedState): AppProps {
  return {
    loggedIn: !!state.primary.userInfo,
    initializing: !state.primary.userInfo && !!state.primary.accessToken
  };
};

export default connect(mapStateToProps)(App);
