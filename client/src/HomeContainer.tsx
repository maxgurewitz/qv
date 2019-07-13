import React from 'react';
import { Dispatch } from 'redux';
import LoadingPage from './LoadingPage';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Action, CombinedState, UserInfo, Polls, InviteIds } from './types';

interface HomeState {
  initialized: boolean
}

class Home extends React.Component<HomeProps, HomeState> {
  componentDidMount() {
    this.props.initialize();
  }

  constructor(props: HomeProps) {
    super(props);
    this.state = {
      initialized: false
    };
  }

  render() {
    return (
      <>
      </>
    );
  }
}

type HomeProps = HomeStateProps & HomeDispatchProps;

interface HomeStateProps {
  userInfo: UserInfo | null,
  polls: Polls,
  inviteIds: InviteIds
}

const mapStateToProps: MapStateToPropsParam<HomeStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
    userInfo: state.primary.userInfo,
    polls: state.primary.polls,
    inviteIds: state.primary.inviteIds
  };
};

interface HomeDispatchProps {
  initialize: () => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    initialize() {
      dispatch({
        source: 'internal',
        type: 'RequestHomeResource',
      });
    }
  };
}

export default connect<HomeStateProps, HomeDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(Home);