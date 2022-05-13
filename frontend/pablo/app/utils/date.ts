export const getHumanizedDateDiff = (timestamp1: number, timestamp2: number) => {
  const diff = Math.abs(timestamp2 - timestamp1);
  const diffDays = diff/1000/3600/24;
  const diffMonths = diffDays/31;

  if (diffDays > 31){
    return diffMonths.toFixed(1) + " months";
  }else{
    return diffDays.toFixed(1) + " days";
  }
};

export const getFullHumanizedDateDiff = (timestamp1: number, timestamp2: number) => {
  let diff = Math.abs(timestamp2 - timestamp1);
  const days = Math.floor(diff/(1000*3600*24));
  diff = diff % (1000*3600*24)
  const hours = Math.floor(diff/(1000*3600));
  diff = diff % (1000 * 3600);
  const minutes = Math.floor(diff/(1000*60));
  const seconds = Math.floor(diff % (1000 * 60) / 1000);

  return (days > 0 ? days + ":" : "") 
          + (hours < 10 ? "0" + hours : hours) + ":"
          + (minutes < 10 ? "0" + minutes : minutes) + ":"
          + (seconds < 10 ? "0" + seconds : seconds)
};
