import React, { useCallback, useState, SetStateAction } from 'react';
import _ from 'lodash';
import { useSelector, useDispatch, connect, MapStateToPropsParam } from 'react-redux';
import TextField from '@material-ui/core/TextField';
import ApiError from './ApiError';
import { CombinedState, Poll, RequestStatus } from './types';

interface PollForm {
  fullDescriptionLink: string | null,
  title: string | null,
  summary: string | null,
}

interface NewPollFormApplicationState {
  createPollRequest: RequestStatus<ApiError, Poll>,
}

function initializePollForm(): PollForm {
  return {
    fullDescriptionLink: null,
    title: null,
    summary: null
  };
}

const NewPoll2: React.FC = () => {
  const dispatch = useDispatch();
  const [form, updateForm] = useState<PollForm>(initializePollForm);

  const newPollFormApplicationState =
    useSelector<CombinedState, NewPollFormApplicationState>((state) => ({
      createPollRequest: state.primary.createPollRequest
    }));

  const createPoll = useCallback(
    () => dispatch({ type: 'create-poll' }),
    [dispatch]
  );

  const isPollValid = !_.isEmpty(form.title) && !_.isEmpty(form.summary);

  return (
    <div>
      poll form

      <form noValidate autoComplete='off'>
        <TextField
          label='Title'
          required
          value={form.title === null ? '' : form.title}
          onChange={e => {
            const { value } = e.target;
            updateForm((form: PollForm) => {
              form.title = value;
              return form;
            })
          }}
          margin='normal'
        />

        <TextField
          label='Summary'
          required
          value={form.summary === null ? '' : form.summary}
          onChange={e => {
            const { value } = e.target;
            updateForm((form: PollForm) => {
              form.summary = value;
              return form;
            })
          }}
          margin='normal'
        />

        <TextField
          label='Full Description Link'
          value={form.fullDescriptionLink === null ? '' : form.fullDescriptionLink}
          onChange={e => {
            const { value } = e.target;
            updateForm((form: PollForm) => {
              form.fullDescriptionLink = value;
              return form;
            })
          }}
          margin='normal'
        />
      </form>
      {/* FIXME submit button should create poll then re-direct to update poll */}
    </div>
  );
};

export default NewPoll2;

// interface ProposalForm {
//   summary: string | null,
//   fullDescriptionLink: string | null,
// }

// interface UserInviteForm {
//   email: string | null,
// }

// interface PollForm {
//   fullDescriptionLink: string | null,
//   title: string | null,
//   summary: string | null,
// }

// interface NewPollState {
//   initializationUuid: string,
//   proposalForms: ProposalForm[],
//   userInviteForms: UserInviteForm[],
//   pollForm: PollForm
// }

// function buildNewUserInviteForm() {
//   return {
//     email: null,
//   };
// }

// function buildNewProposalForm() {
//   return {
//     summary: null,
//     fullDescriptionLink: null
//   };
// }

// function backButton(url: string) {
//   return (
//     <Button variant='contained' >
//       <Link color='inherit' component={RouterLink} to={url}>
//         Back
//       </Link>
//     </Button>
//   );
// }

// function nextButton(enabled: boolean, url: string) {
//   return (
//     <Button color={enabled ? 'primary' : 'secondary'} disabled={!enabled} variant='contained'>
//       <Link color='inherit' component={RouterLink} to={url}>
//         Next
//       </Link>
//     </Button>
//   );
// }

// type OnChangeHandlerFunction = (value: string, draft: NewPollState) => NewPollState;
// type OnChangeHandlerSupplier = (draft: NewPollState) => NewPollState;
// type ChangeEventHandler = React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>;
// type MouseEventHandler = React.MouseEventHandler;

// class NewPoll extends React.Component<NewPollProps, NewPollState> {

//   buildOnInputHandler(onChangeHandler: OnChangeHandlerFunction): ChangeEventHandler {
//     return event => {
//       const value = event.target.value;
//       this.setState(state =>
//         produce(state, draft =>
//           onChangeHandler(value, draft)
//         )
//       )
//     };
//   }

//   buildOnClickHandler(onChangeHandler: OnChangeHandlerSupplier): MouseEventHandler {
//     return () => {
//       this.setState(state => {
//         return produce(state, draft =>
//           onChangeHandler(draft)
//         )
//       })
//     };
//   }

//   constructor(props: NewPollProps) {
//     super(props);

//     this.state = {
//       initializationUuid: uuid(),
//       userInviteForms: [buildNewUserInviteForm()],
//       proposalForms: [buildNewProposalForm()],
//       pollForm: {
//         fullDescriptionLink: null,
//         title: null,
//         summary: null
//       }
//     };
//     this.props.initialize(this.state.initializationUuid);
//   }

//   renderProposals() {
//     const areAllProposalsValid = _.every(this.state.proposalForms, (propsalForm: ProposalForm) => !_.isEmpty(propsalForm.summary));

//     return (
//       <div>
//         proposals form
//         <form>
//           {_.map(this.state.proposalForms, (proposalForm: ProposalForm, i: number) => (
//             <div key={i}>
//               <TextField
//                 label='Summary'
//                 required
//                 value={proposalForm.summary === null ? '' : proposalForm.summary}
//                 onChange={
//                   this.buildOnInputHandler((summary, draft) => {
//                     draft.proposalForms[i].summary = summary;
//                     return draft;
//                   })
//                 }
//                 margin='normal'
//               />
//               <TextField
//                 label='Full Description Link'
//                 value={proposalForm.fullDescriptionLink === null ? '' : proposalForm.fullDescriptionLink}
//                 onChange={
//                   this.buildOnInputHandler((fullDescriptionLink, draft) => {
//                     draft.proposalForms[i].fullDescriptionLink = fullDescriptionLink;
//                     return draft;
//                   })
//                 }
//                 margin='normal'
//               />

//               {i > 0 ?
//                 <IconButton onClick={this.buildOnClickHandler((draft) => {
//                   draft.proposalForms.splice(i, 1);
//                   return draft;
//                 })}>
//                   <Icon color='primary'> remove_circle </Icon>
//                 </IconButton> :
//                 <React.Fragment />
//               }
//             </div>
//           ))}
//         </form>

//         <IconButton onClick={this.buildOnClickHandler((draft) => {
//           draft.proposalForms.push(buildNewProposalForm());
//           return draft;
//         })}>
//           <Icon color='primary'> add_circle </Icon>
//         </IconButton>

//         {backButton('/new-poll')}
//         {nextButton(areAllProposalsValid, '/new-poll/user-invites')}
//       </div>
//     );
//   }

//   renderUserInvites() {
//     const areAllInvitesValid = _.every(this.state.userInviteForms, propsalForm => !_.isEmpty(propsalForm.email));

//     return (
//       <div>user invites form
//         <form>
//           {_.map(this.state.userInviteForms, (userInviteForm, i) => (
//             <div key={i}>
//               <TextField
//                 label='Email'
//                 required
//                 value={userInviteForm.email === null ? '' : userInviteForm.email}
//                 onChange={
//                   this.buildOnInputHandler((email, draft) => {
//                     draft.userInviteForms[i].email = email;
//                     return draft;
//                   })
//                 }
//                 margin='normal'
//               />

//               {i > 0 ?
//                 <IconButton onClick={this.buildOnClickHandler((draft) => {
//                   draft.userInviteForms.splice(i, 1);
//                   return draft;
//                 })}>
//                   <Icon color='primary'> remove_circle </Icon>
//                 </IconButton> :
//                 <React.Fragment />
//               }
//             </div>
//           ))}

//           <IconButton onClick={this.buildOnClickHandler((draft) => {
//             draft.userInviteForms.push(buildNewUserInviteForm());
//             return draft;
//           })}>
//             <Icon color='primary'> add_circle </Icon>
//           </IconButton>
//         </form>
//         {backButton('/new-poll/proposals')}
//         {nextButton(areAllInvitesValid, '/new-poll/finished')}
//       </div>
//     );
//   }

//   renderNewPoll() {
//     const isPollValid = !_.isEmpty(this.state.pollForm.title) && !_.isEmpty(this.state.pollForm.summary);

//     return (
//       <div>
//         poll form

//         <form noValidate autoComplete='off'>
//           <TextField
//             label='Title'
//             required
//             value={this.state.pollForm.title === null ? '' : this.state.pollForm.title}
//             onChange={
//               this.buildOnInputHandler((title, draft) => {
//                 draft.pollForm.title = title;
//                 return draft;
//               })
//             }
//             margin='normal'
//           />

//           <TextField
//             label='Summary'
//             required
//             value={this.state.pollForm.summary === null ? '' : this.state.pollForm.summary}
//             onChange={
//               this.buildOnInputHandler((summary, draft) => {
//                 draft.pollForm.summary = summary;
//                 return draft;
//               })
//             }
//             margin='normal'
//           />

//           <TextField
//             label='Full Description Link'
//             value={this.state.pollForm.fullDescriptionLink === null ? '' : this.state.pollForm.fullDescriptionLink}
//             onChange={
//               this.buildOnInputHandler((fullDescriptionLink, draft) => {
//                 draft.pollForm.fullDescriptionLink = fullDescriptionLink;
//                 return draft;
//               })
//             }
//             margin='normal'
//           />
//         </form>
//         {nextButton(isPollValid, '/new-poll/proposals')}
//       </div>
//     );
//   }

//   render() {
//     return (
//       <div>
//         <Switch>
//           <Route path='/new-poll/proposals' render={() => this.renderProposals()} />
//           <Route path='/new-poll/user-invites' render={() => this.renderUserInvites()} />
//           <Route path='/new-poll/finished' render={() => (
//             <div>you have completed your poll</div>
//           )} />
//           <Route path='/new-poll' render={() => this.renderNewPoll()} />
//         </Switch>
//       </div>
//     );
//   }
// }