import React from 'react';
import { Dispatch } from 'redux';
import Grid from '@material-ui/core/Grid';
import CircularProgress from '@material-ui/core/CircularProgress';
import { connect, MapStateToPropsParam } from 'react-redux';
import { State, Action, CombinedState } from './types';
import { access } from 'fs';
import { any } from 'prop-types';
import { logOut } from './auth';

class AuthCallback extends React.Component<AuthCallbackProps, {}> {

  componentDidMount() {
    const { state, logOut, accessToken, getUserInfo } = this.props;

    if (accessToken != null && state != null) {
      getUserInfo(accessToken, state);
    } else {
      debugger;
      logOut();
    }
  }

  render() {
    return (
      <Grid
        container
        direction="column"
        justify="center"
        alignItems="center"
        className="centered-grid">
        <CircularProgress />
      </Grid>
    );
  }
}

interface AuthCallbackStateProps {
  accessToken: string | null,
  state: string | null,
}

type AuthCallbackProps = AuthCallbackStateProps & AuthCallbackDispatchProps;

const mapStateToProps: MapStateToPropsParam<AuthCallbackStateProps, {}, CombinedState> = (state: CombinedState) => {
  const hash = state.router.location.hash;
  const queryParams = new URLSearchParams(hash.slice(1, hash.length));

  debugger;
  return {
    accessToken: queryParams.get('access_token'),
    state: queryParams.get('state'),
  };
};

interface AuthCallbackDispatchProps {
  getUserInfo: (accessToken: string, state: string) => void
  logOut: () => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    getUserInfo: (accessToken: string, state: string) => {
      dispatch({
        source: 'internal',
        type: 'AuthCallback',
        accessToken,
        state
    });
    },
    logOut: () => {
      dispatch({
        source: 'internal',
        type: 'LogOut',
      });
    }
  };
}

export default connect<AuthCallbackStateProps, AuthCallbackDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(AuthCallback);