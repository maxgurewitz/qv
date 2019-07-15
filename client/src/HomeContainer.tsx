import React from 'react';
import uuid from 'uuid/v4';
import _ from 'lodash';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Dispatch } from 'redux';
import Container from '@material-ui/core/Container';
import CircularProgress from '@material-ui/core/CircularProgress';
import { Action, CombinedState, UserInfo, Polls, InviteIds } from './types';
import styles from './Home.module.css';

interface HomeState {
  initializationUuid: string
}

class Home extends React.Component<HomeProps, HomeState> {

  constructor(props: HomeProps) {
    super(props);
    this.state = {
      initializationUuid: uuid()
    };
    this.props.initialize(this.state.initializationUuid);
  }

  render() {
    const isLoading = this.props.requestsInFlight.has(this.state.initializationUuid);
    if (isLoading) {
      return (
        <div className={styles.loading}>
          <CircularProgress/>
        </div>
      );
    }

    const adminPolls = _.filter(this.props.polls, (poll) => 
      poll !== null && this.props.userInfo !== null && poll.email === this.props.userInfo.email
    );

    // TODO switch invite ids to set
    const invitePollIds = this.props.userInfo !== null ?
      this.props.inviteIds[this.props.userInfo.email] :
      [];

    const invitePolls = invitePollIds.map(id => this.props.polls[id]).filter(poll => !!poll);

    return (
      <Container maxWidth="sm">
      </Container>
    );
  }
}

type HomeProps = HomeStateProps & HomeDispatchProps;

interface HomeStateProps {
  userInfo: UserInfo | null,
  polls: Polls,
  inviteIds: InviteIds,
  requestsInFlight: Set<String>
}

const mapStateToProps: MapStateToPropsParam<HomeStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
    userInfo: state.primary.userInfo,
    polls: state.primary.polls,
    requestsInFlight: state.primary.requestsInFlight,
    inviteIds: state.primary.inviteIds
  };
};

interface HomeDispatchProps {
  initialize: (initializationUuid: string) => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    initialize(initializationUuid: string) {
      dispatch({
        source: 'internal',
        type: 'RequestHomeResource',
        uuid: initializationUuid
      });
    }
  };
}

export default connect<HomeStateProps, HomeDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(Home);