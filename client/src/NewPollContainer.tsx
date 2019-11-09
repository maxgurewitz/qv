import React from 'react';
import { useImmer } from 'use-immer';
import { Draft } from 'immer';
import uuid from 'uuid/v4';
import _ from 'lodash';
import { connect, MapStateToPropsParam } from 'react-redux';
import { Dispatch } from 'redux';
import { Route, Switch } from 'react-router';
import { Link as RouterLink } from 'react-router-dom';
import Icon from '@material-ui/core/Icon';
import IconButton from '@material-ui/core/IconButton';
import Link from '@material-ui/core/Link';
import produce from 'immer';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import ApiError from './ApiError';
import { Action, CombinedState, Poll, RequestStatus } from './types';

interface NewPollFormProps2 {
  createPollRequest: RequestStatus<ApiError, Poll>,
}

function initializePollForm(): PollForm {
  return {
    fullDescriptionLink: null,
    title: null,
    summary: null
  };
}

function createFieldUpdator<S>(
  immerUpdator: ImmerUpdator<S>,
  update: (targetValue: string, draft: Draft<S>) => void) {

  return (event: ChangeEvent) => {
    return immerUpdator(draft => {
      update(event.target.value, draft);
    });
  };
}

type ImmerUpdator<S> = (f: (draft: Draft<S>) => void | S) => void;
type ChangeEvent = React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>;

function updateTitle(title: string, draft: Draft<PollForm>) {
  draft.title = title;
};

function updateFullDescriptionLink(fullDescriptionLink: string, draft: Draft<PollForm>) {
  draft.fullDescriptionLink = fullDescriptionLink;
}

function updateSummary(summary: string, draft: Draft<PollForm>) {
  draft.summary = summary;
}

// FIXME get useImmer
const NewPollForm2: React.FC<NewPollFormProps2> = ({ createPollRequest }) => {
  const [form, updateForm] = useImmer<PollForm>(initializePollForm);

  const isPollValid = !_.isEmpty(form.title) && !_.isEmpty(form.summary);

  return (
    <div>
      poll form

      <form noValidate autoComplete='off'>
        <TextField
          label='Title'
          required
          value={form.title === null ? '' : form.title}
          onChange={createFieldUpdator(updateForm, updateTitle)}
          margin='normal'
        />

        <TextField
          label='Summary'
          required
          value={form.summary === null ? '' : form.summary}
          onChange={createFieldUpdator(updateForm, updateSummary)}
          margin='normal'
        />

        <TextField
          label='Full Description Link'
          value={form.fullDescriptionLink === null ? '' : form.fullDescriptionLink}
          onChange={createFieldUpdator(updateForm, updateFullDescriptionLink)}
          margin='normal'
        />
      </form>
      {/* FIXME should create poll then re-direct to update poll */}
      {nextButton(isPollValid, '/new-poll/proposals')}
    </div>
  );
};

interface ProposalForm {
  summary: string | null,
  fullDescriptionLink: string | null,
}

interface UserInviteForm {
  email: string | null,
}

interface PollForm {
  fullDescriptionLink: string | null,
  title: string | null,
  summary: string | null,
}

interface NewPollState {
  initializationUuid: string,
  proposalForms: ProposalForm[],
  userInviteForms: UserInviteForm[],
  pollForm: PollForm
}

function buildNewUserInviteForm() {
  return {
    email: null,
  };
}

function buildNewProposalForm() {
  return {
    summary: null,
    fullDescriptionLink: null
  };
}

function backButton(url: string) {
  return (
    <Button variant='contained' >
      <Link color='inherit' component={RouterLink} to={url}>
        Back
      </Link>
    </Button>
  );
}

function nextButton(enabled: boolean, url: string) {
  return (
    <Button color={enabled ? 'primary' : 'secondary'} disabled={!enabled} variant='contained'>
      <Link color='inherit' component={RouterLink} to={url}>
        Next
      </Link>
    </Button>
  );
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
    return () => {
      this.setState(state => {
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
      userInviteForms: [buildNewUserInviteForm()],
      proposalForms: [buildNewProposalForm()],
      pollForm: {
        fullDescriptionLink: null,
        title: null,
        summary: null
      }
    };
    this.props.initialize(this.state.initializationUuid);
  }

  renderProposals() {
    const areAllProposalsValid = _.every(this.state.proposalForms, (propsalForm: ProposalForm) => !_.isEmpty(propsalForm.summary));

    return (
      <div>
        proposals form
        <form>
          {_.map(this.state.proposalForms, (proposalForm: ProposalForm, i: number) => (
            <div key={i}>
              <TextField
                label='Summary'
                required
                value={proposalForm.summary === null ? '' : proposalForm.summary}
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
                value={proposalForm.fullDescriptionLink === null ? '' : proposalForm.fullDescriptionLink}
                onChange={
                  this.buildOnInputHandler((fullDescriptionLink, draft) => {
                    draft.proposalForms[i].fullDescriptionLink = fullDescriptionLink;
                    return draft;
                  })
                }
                margin='normal'
              />

              {i > 0 ?
                <IconButton onClick={this.buildOnClickHandler((draft) => {
                  draft.proposalForms.splice(i, 1);
                  return draft;
                })}>
                  <Icon color='primary'> remove_circle </Icon>
                </IconButton> :
                <React.Fragment />
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

        {backButton('/new-poll')}
        {nextButton(areAllProposalsValid, '/new-poll/user-invites')}
      </div>
    );
  }

  renderUserInvites() {
    const areAllInvitesValid = _.every(this.state.userInviteForms, propsalForm => !_.isEmpty(propsalForm.email));

    return (
      <div>user invites form
        <form>
          {_.map(this.state.userInviteForms, (userInviteForm, i) => (
            <div key={i}>
              <TextField
                label='Email'
                required
                value={userInviteForm.email === null ? '' : userInviteForm.email}
                onChange={
                  this.buildOnInputHandler((email, draft) => {
                    draft.userInviteForms[i].email = email;
                    return draft;
                  })
                }
                margin='normal'
              />

              {i > 0 ?
                <IconButton onClick={this.buildOnClickHandler((draft) => {
                  draft.userInviteForms.splice(i, 1);
                  return draft;
                })}>
                  <Icon color='primary'> remove_circle </Icon>
                </IconButton> :
                <React.Fragment />
              }
            </div>
          ))}

          <IconButton onClick={this.buildOnClickHandler((draft) => {
            draft.userInviteForms.push(buildNewUserInviteForm());
            return draft;
          })}>
            <Icon color='primary'> add_circle </Icon>
          </IconButton>
        </form>
        {backButton('/new-poll/proposals')}
        {nextButton(areAllInvitesValid, '/new-poll/finished')}
      </div>
    );
  }

  renderNewPoll() {
    const isPollValid = !_.isEmpty(this.state.pollForm.title) && !_.isEmpty(this.state.pollForm.summary);

    return (
      <div>
        poll form

        <form noValidate autoComplete='off'>
          <TextField
            label='Title'
            required
            value={this.state.pollForm.title === null ? '' : this.state.pollForm.title}
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
            value={this.state.pollForm.summary === null ? '' : this.state.pollForm.summary}
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
            value={this.state.pollForm.fullDescriptionLink === null ? '' : this.state.pollForm.fullDescriptionLink}
            onChange={
              this.buildOnInputHandler((fullDescriptionLink, draft) => {
                draft.pollForm.fullDescriptionLink = fullDescriptionLink;
                return draft;
              })
            }
            margin='normal'
          />
        </form>
        {nextButton(isPollValid, '/new-poll/proposals')}
      </div>
    );
  }

  render() {
    return (
      <div>
        <Switch>
          <Route path='/new-poll/proposals' render={() => this.renderProposals()} />
          <Route path='/new-poll/user-invites' render={() => this.renderUserInvites()} />
          <Route path='/new-poll/finished' render={() => (
            <div>you have completed your poll</div>
          )} />
          <Route path='/new-poll' render={() => this.renderNewPoll()} />
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