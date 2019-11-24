import React from 'react';
import uuid from 'uuid/v4';
import _ from 'lodash';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Dispatch } from 'redux';
import CircularProgress from '@material-ui/core/CircularProgress';
import List from '@material-ui/core/List';
import Button from '@material-ui/core/Button';
import ListItem from '@material-ui/core/ListItem';
import Link from '@material-ui/core/Link';
import { Link as RouterLink, Redirect } from 'react-router-dom';
// import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import { RequestStatus, Poll, Action, CombinedState, UserInfo, Polls, InvitePollIds } from './types';
import styles from './Home.module.css';
import { AxiosError } from 'axios';

interface HomeState { }

class Home extends React.Component<HomeProps, HomeState> {

  constructor(props: HomeProps) {
    super(props);
    this.state = {
      initializationUuid: uuid()
    };
    this.props.initialize();
  }

  render() {
    const isLoading = this.props.initializeRequest.type === 'InProgressRequestStatus';

    if (isLoading) {
      return (
        <div className={styles.loading}>
          <CircularProgress />
        </div>
      );
    }

    const isError = this.props.initializeRequest.type === 'FailedRequestStatus';

    // TODO error handling on initialization request
    if (isError) {
      return (<div>error</div>);
    }

    const adminPolls = _.filter(this.props.polls, (poll) =>
      poll !== null &&
      this.props.userInfo !== null &&
      poll.email === this.props.userInfo.email
    ) as Poll[];

    // TODO switch invite ids to ordered set
    const invitePollIds = this.props.userInfo !== null ?
      (this.props.invitePollIds[this.props.userInfo.email] || []) :
      [];

    const invitePolls = invitePollIds
      .map(id => this.props.polls[id])
      .filter(poll => !!poll) as Poll[];

    return (
      <div>
        <Button className={styles.createPoll} variant="contained">
          <Link component={RouterLink} to="/new-poll">
            Create Poll
          </Link>
        </Button>
        <List>
          {
            _.map(adminPolls, (poll, i) =>
              (
                <ListItem key={i}>
                  <ListItemText primary={poll.title} />
                </ListItem>
              )
            )
          }
        </List>
        <List>
          {
            _.map(invitePolls, (poll, i) =>
              (
                <ListItem key={i}>
                  <ListItemText primary={poll.title} />
                </ListItem>
              )
            )
          }
        </List>
      </div>
    );
  }
}

type HomeProps = HomeStateProps & HomeDispatchProps;

interface HomeStateProps {
  userInfo: UserInfo | null,
  polls: Polls,
  invitePollIds: InvitePollIds,
  initializeRequest: RequestStatus<void, AxiosError>,
}

const mapStateToProps: MapStateToPropsParam<HomeStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
    userInfo: state.primary.userInfo,
    polls: state.primary.polls,
    initializeRequest: state.primary.initializeRequest,
    invitePollIds: state.primary.invitePollIds
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