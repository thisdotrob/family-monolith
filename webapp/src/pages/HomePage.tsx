import React from 'react';
import { useApolloClient, useQuery } from '@apollo/client';
import { useAuth } from '../contexts/AuthContext';
import { ME_QUERY } from '../graphql/queries';

const HomePage = () => {
  const { logout } = useAuth();

  const client = useApolloClient();

  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  const logoutOnClick = async () => {
    try {
      await logout();
      client.clearStore();
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md w-full max-w-md text-center">
        <h1 className="text-2xl font-bold mb-6">Welcome, {data?.me?.firstName || 'User'}!</h1>
        <p>Your username is: {data?.me?.username}</p>
        <button
          onClick={logoutOnClick}
          className="mt-6 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
        >
          Logout
        </button>
      </div>
    </div>
  );
};

export default HomePage;
