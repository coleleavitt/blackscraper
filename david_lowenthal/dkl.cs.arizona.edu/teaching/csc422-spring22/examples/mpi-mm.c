// CSc 422
// MPI matrix multiplication example
// Assumes that number of MPI processes divides N

#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>

#define TAG 13

double **allocateMatrix(int N, int M) {
  int i;
  double *vals, **temp;

  // allocate values
  vals = (double *) malloc (N * M * sizeof(double));

  // allocate vector of pointers
  temp = (double **) malloc (N * sizeof(double*));

  for(i=0; i < N; i++)
    temp[i] = &(vals[i * M]);

  return temp;
}

void printMatrix(double **mat, int N, int M) {
  int i,j;

  printf("The %d * %d matrix is\n", N, M);
  for(i=0; i < N; i++){
    for(j=0; j < M; j++)
      printf("%lf ",  mat[i][j]);
    printf("\n");
  }
}

int main(int argc, char *argv[]) {
  double **A, **B, **C;
  double startTime, endTime, sum;
  int numElements, offset, stripSize, myId, numProcesses, N, i, j, k;
  
  MPI_Init(&argc, &argv);
  
  MPI_Comm_rank(MPI_COMM_WORLD, &myId);
  MPI_Comm_size(MPI_COMM_WORLD, &numProcesses);
  
  N = atoi(argv[1]);
  
  // allocate A, B, and C --- note that you need these to be
  // contiguously allocated.  Workers need less memory allocated.
  
  if (myId == 0) {
    A = allocateMatrix(N, N);
    C = allocateMatrix(N, N);
  }
  else {
    A = allocateMatrix(N/numProcesses, N);
    C = allocateMatrix(N/numProcesses, N);
  }
  
  B = allocateMatrix(N, N);
  
  if (myId == 0) {
    // initialize A and B
    for (i = 0; i < N; i++) {
      for (j = 0; j < N; j++) {
        A[i][j] = i+j;
        B[i][j] = i+j;
      }
    }
  }

  // print out matrices here, if I'm the administrator
  if (myId == 0 && N < 10) {
    printMatrix(A, N, N);
    printMatrix(B, N, N);
  }
  
  // start timer
  if (myId == 0) {
    startTime = MPI_Wtime();
  }
  
  stripSize = N/numProcesses;

  // send each process its piece of A -- note could be done via MPI_Scatter
  // note that if MPI_Scatter is used, it would be invoked as follows:
  // MPI_Scatter(A[0], stripSize * N, MPI_DOUBLE, A[0], stripSize * N, MPI_DOUBLE, 0, MPI_COMM_WORLD);
  if (myId == 0) {
    offset = stripSize;
    numElements = stripSize * N;
    for (i = 1; i < numProcesses; i++) {
      MPI_Send(A[offset], numElements, MPI_DOUBLE, i, TAG, MPI_COMM_WORLD);
      offset += stripSize;
    }
  }
  else {  // receive my part of A
    MPI_Recv(A[0], stripSize * N, MPI_DOUBLE, 0, TAG, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
  }
  
  // everyone gets B
  // note that *all* processes execute the broadcast; the root is designated by the 
  //   second to last parameter
  // all MPI collectives are similar in that there are no conditionals for the root
  //   as there are above when distributing matrix A
  MPI_Bcast(B[0], N*N, MPI_DOUBLE, 0, MPI_COMM_WORLD);

  // do the work
  for (i = 0; i < stripSize; i++) {
    for (j = 0; j < N; j++) {
      sum = 0.0;
      for (k = 0; k < N; k++) {
        sum += A[i][k] * B[k][j];
      }
      C[i][j] = sum;
    }
  }

  // administrator receives from workers  -- note could be done via MPI_Gather
  if (myId == 0) {
    offset = stripSize; 
    numElements = stripSize * N;
    for (i=1; i < numProcesses; i++) {
      MPI_Recv(C[offset], numElements, MPI_DOUBLE, i, TAG, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
      offset += stripSize;
    }
  }
  else { // send worker's piece of C to administrator
    MPI_Send(C[0], stripSize * N, MPI_DOUBLE, 0, TAG, MPI_COMM_WORLD);
  }

  // stop timer
  if (myId == 0) {
    endTime = MPI_Wtime();
    printf("Time is %f\n", endTime-startTime);
  }
  
  // print out matrix here, if I'm the administrator
  if (myId == 0 && N < 10) {
    printMatrix(C, N, N);
  }
  
  MPI_Finalize();
  return 0;
}


