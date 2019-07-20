import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import { connect } from 'react-redux';
import { Route, Switch, Redirect } from 'react-router';
import { ConnectedRouter } from 'connected-react-router';
import { history } from './configureStore';
import './App.css';
import Landing from './LandingContainer';
import AuthCallback from './AuthCallbackContainer';
import Initializing from './InitializingContainer';
import LoggedIn from './LoggedInContainer';
import NotFound from './NotFound';
import { CombinedState } from './types';

interface InitializedProps {
  loggedIn: boolean,
}

const Initialized: React.FC<InitializedProps> = ({ loggedIn }) => {
  return (
    <Switch>
      <Route exact path="/" render={() => (
        loggedIn ?
          <Redirect to="/app" /> :
          <Landing />
      )} />
      <Route path="/:path(app|new-poll)" render={() => (
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
    </Switch>
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
