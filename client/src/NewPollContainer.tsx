import React, { SyntheticEvent } from 'react';
import uuid from 'uuid/v4';
import _ from 'lodash';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Dispatch } from 'redux';
import { Route, Switch } from 'react-router';
import {Link as RouterLink} from 'react-router-dom';
import Icon from '@material-ui/core/Icon';
import IconButton from '@material-ui/core/IconButton';
import Link from '@material-ui/core/Link';
import produce from 'immer';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import { Action, CombinedState } from './types';

interface ProposalForm {
  summary: string | null,
  fullDescriptionLink: string | null,
}

interface NewPollState {
  initializationUuid: string,
  proposalForms: ProposalForm[],
  pollForm: {
    fullDescriptionLink: string | null,
    title: string | null,
    summary: string | null,
  }
}

function buildNewProposalForm () {
  return {
    summary: null,
    fullDescriptionLink: null
  };
}

type OnChangeHandlerFunction = (value: string, draft: NewPollState) => NewPollState;
type OnChangeHandlerSupplier = (draft: NewPollState) => NewPollState;
type ChangeEventHandler = React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>;
type MouseEventHandler = React.MouseEventHandler;

class NewPoll extends React.Component<NewPollProps, NewPollState> {

  buildOnInputHandler(onChangeHandler: OnChangeHandlerFunction): ChangeEventHandler {
    return event => {
      const value = event.target.value;
      this.setState(state => 
        produce(state, draft => 
          onChangeHandler(value, draft)
        )
      )
    };
  }

  buildOnClickHandler(onChangeHandler: OnChangeHandlerSupplier): MouseEventHandler {
    console.log('loc2');
    return () => {
      // FIXME not making it here
      console.log('loc3');
      this.setState(state => {
        console.log('loc4');
        return produce(state, draft => 
          onChangeHandler(draft)
        )
      })
    };
  }

  constructor(props: NewPollProps) {
    super(props);
    this.state = {
      initializationUuid: uuid(),
      proposalForms: [buildNewProposalForm()],
      pollForm: {
        fullDescriptionLink: null,
        title: null,
        summary: null
      }
    };
    this.props.initialize(this.state.initializationUuid);
  }

  render() {
    return (
      <div>
        <Switch>
          <Route path='/new-poll/proposals' render={() => {
            const areAllProposalsValid = _.every(this.state.proposalForms, propsalForm => !_.isEmpty(propsalForm.summary)); 

            return (
              <div>
                proposals form
                <form>
                {_.map(this.state.proposalForms, (proposalForm, i) => (
                  <div key={i}>
                    <TextField
                      label='Summary'
                      required
                      value={ proposalForm.summary === null ? '' : proposalForm.summary }
                      onChange={
                        this.buildOnInputHandler((summary, draft) => {
                          draft.proposalForms[i].summary = summary;
                          return draft;
                        })
                      }
                      margin='normal'
                    />
                    <TextField
                      label='Full Description Link'
                      value={ proposalForm.fullDescriptionLink === null ? '' : proposalForm.fullDescriptionLink }
                      onChange={
                        this.buildOnInputHandler((fullDescriptionLink, draft) => {
                          draft.proposalForms[i].fullDescriptionLink = fullDescriptionLink;
                          return draft;
                        })
                      }
                      margin='normal'
                    />

                    { i > 0 ?
                      <IconButton onClick={this.buildOnClickHandler((draft) => {
                        draft.proposalForms.splice(i, 1);
                        return draft;
                      })}>
                        <Icon color='primary'> remove_circle </Icon>
                      </IconButton> : 
                      <React.Fragment/>
                    }
                  </div>
                ))}
                </form>

                <IconButton onClick={this.buildOnClickHandler((draft) => {
                  draft.proposalForms.push(buildNewProposalForm());
                  return draft;
                })}>
                  <Icon color='primary'> add_circle </Icon>
                </IconButton>

                <Button variant='contained' >
                  <Link color='inherit' component={RouterLink} to='/new-poll'>
                    Back
                  </Link>
                </Button> 

                <Button color={areAllProposalsValid ? 'primary' : 'secondary'} disabled={!areAllProposalsValid} variant='contained'>
                  <Link color='inherit' component={RouterLink} to='/new-poll/user-invites'>
                    Next
                  </Link>
                </Button> 
              </div>
            );
          }}/>
          <Route path='/new-poll/user-invites' render={() => (
            <div>user invites form</div>
          )}/>
          <Route path='/new-poll/finished' render={() => (
            <div>you have completed your poll</div>
          )}/>
          <Route path='/new-poll' render={() => {
            const isPollValid = !_.isEmpty(this.state.pollForm.title) && !_.isEmpty(this.state.pollForm.summary);

            return (
              <div>
                poll form

                <form noValidate autoComplete='off'>
                  <TextField
                    label='Title'
                    required
                    value={ this.state.pollForm.title === null ? '' : this.state.pollForm.title }
                    onChange={
                      this.buildOnInputHandler((title, draft) => {
                          draft.pollForm.title = title;
                          return draft;
                      })
                    }
                    margin='normal'
                  />

                  <TextField
                    label='Summary'
                    required
                    value={this.state.pollForm.summary === null ? '' : this.state.pollForm.summary }
                    onChange={
                      this.buildOnInputHandler((summary, draft) => {
                          draft.pollForm.summary = summary;
                          return draft;
                      })
                    }
                    margin='normal'
                  />

                  <TextField
                    label='Full Description Link'
                    value={this.state.pollForm.fullDescriptionLink === null ? '' : this.state.pollForm.fullDescriptionLink }
                    onChange={
                      this.buildOnInputHandler((fullDescriptionLink, draft) => {
                          draft.pollForm.fullDescriptionLink = fullDescriptionLink;
                          return draft;
                      })
                    }
                    margin='normal'
                  />
                </form>
                <Button color={isPollValid ? 'primary' : 'secondary'} disabled={!isPollValid} variant='contained'>
                  <Link color='inherit' component={RouterLink} to='/new-poll/proposals'>
                    Next
                  </Link>
                </Button> 
              </div>
            );
          }}/>
        </Switch>
      </div>
    );
  }
}

type NewPollProps = NewPollStateProps & NewPollDispatchProps;

interface NewPollStateProps {
  requestsInFlight: Set<String>
}

const mapStateToProps: MapStateToPropsParam<NewPollStateProps, {}, CombinedState> = (state: CombinedState) => {
  return {
    requestsInFlight: state.primary.requestsInFlight,
  };
};

interface NewPollDispatchProps {
  initialize: (initializationUuid: string) => void
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
  return {
    // TODO memoize id of WIP poll to come back to it
    initialize(initializationUuid: string) {
    }
  };
}

export default connect<NewPollStateProps, NewPollDispatchProps, {}, CombinedState>(mapStateToProps, mapDispatchToProps)(NewPoll);