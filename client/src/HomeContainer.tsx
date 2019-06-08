import React from 'react';
import { connect } from 'react-redux';
import { Dispatch } from 'redux';
import Grid from '@material-ui/core/Grid';
import Button from '@material-ui/core/Button';
import { login } from './auth';
import { Action } from './types';

const Home: React.FC = () => {
    return (
        <Grid
            container
            direction="column"
            justify="center"
            alignItems="center"
            className="centered-grid">
            <div className="Home">
                <Button onClick={login} variant="contained" color="primary" >
                    Login
                </Button>
            </div>
        </Grid>
    );
}

function mapDispatchToProps(dispatch: Dispatch<Action>) {
    return {
        login: () => dispatch({
            source: 'internal',
            type: 'Login'
        })
    };
}

export default connect(null, mapDispatchToProps)(Home);