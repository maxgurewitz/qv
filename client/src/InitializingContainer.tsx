import React from 'react';
import { Dispatch } from 'redux';
import LoadingPage from './LoadingPage';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Action, CombinedState } from './types';

class Initializing extends React.Component<InitializingProps, {}> {

  componentDidMount() {
    const { accessToken, getUserInfo } = this.props;
    getUserInfo(accessToken);
  }

  render() {
    return (
      <LoadingPage/>
    );
  }
}

interface InitializingStateProps {
  accessToken: string | null,
}

type InitializingProps = InitializingStateProps & InitializingDispatchProps;

const mapStateToProps: MapStateToPropsParam<InitializingStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
    accessToken: state.primary.accessToken,
  };
};

interface InitializingDispatchProps {
  getUserInfo: (accessToken: string | null) => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    getUserInfo: (accessToken: string | null) => {
      dispatch({
        source: 'internal',
        type: 'Initialize',
        accessToken
      });
    },
  }
}

export default connect<InitializingStateProps, InitializingDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(Initializing);