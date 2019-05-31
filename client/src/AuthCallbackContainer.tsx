import React from 'react';
import { Dispatch } from 'redux';
import Grid from '@material-ui/core/Grid';
import CircularProgress from '@material-ui/core/CircularProgress';
import { Action } from './types';

class AuthCallback extends React.Component {
    componentDidMount() {

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

function mapDispatchToProps(dispatch: Dispatch<Action>) {
    return {
        // login: () => dispatch({
        //     source: 'internal',
        //     type: 'Login'
        // })
    };
}