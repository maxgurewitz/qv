import React from 'react';
import { Dispatch } from 'redux';
import LoadingPage from './LoadingPage';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Action, CombinedState } from './types';

class AuthCallback extends React.Component<AuthCallbackProps, {}> {

  componentDidMount() {
    const { state, logOut, accessToken, getUserInfo } = this.props;

    if (accessToken != null && state != null) {
      getUserInfo(accessToken, state);
    } else {
      logOut();
    }
  }

  render() {
    return (
      <LoadingPage/>
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