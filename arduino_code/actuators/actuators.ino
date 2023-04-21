 #include <Servo.h>

// VARS

int SerialTimeout = 5;
String msg;

const int numReadings = 2;

int timeSlotMillis = 5;
int timeSoFar = 0;
int prevTime = 0;
int maxVib = 100;

#define whPin 2
int readings_wh[numReadings];  // wheel
int readIndex_wh = 0;          // the index of the current reading
int total_wh = 0;              // the running total
int average_wh = 0;            // the average
int whVibLevel = 0;
int whON = 20; // this can never be 0
int whOFF = 100;

#define bkPin 3
int readings_bk[numReadings];  // back(seat)
int readIndex_bk = 0;          // the index of the current reading
int total_bk = 0;              // the running total
int average_bk = 0;            // the average
int bkVibLevel = 0;
int bkON = 20; // this can never be 0
int bkOFF = 10;




void actuate(int vib, int wait) {
  digitalWrite(whPin,HIGH);
  delay(vib);
  digitalWrite(whPin, LOW);
  delay(wait); 
}

int getVib(int vibSpeed) {
  int result;
  if ((vibSpeed >= 0) && (vibSpeed < 10)) {
    result = 55;
  } else if ((vibSpeed >= 10) && (vibSpeed < 20)) {
    result = 50;
  } else if ((vibSpeed >= 20) && (vibSpeed < 30)) {
    result = 40;
  } else if ((vibSpeed >= 30) && (vibSpeed < 40)) {
    result = 30;
  } else if ((vibSpeed >= 40) && (vibSpeed < 50)) {
    result = 25;
  } else if ((vibSpeed >= 50) && (vibSpeed < 60)) {
    result = 20;
  } else if ((vibSpeed >= 60) && (vibSpeed < 70)) {
    result = 15;
  } else if ((vibSpeed >= 70) && (vibSpeed < 80)) {
    result = 10;
  } else if ((vibSpeed >= 80) && (vibSpeed < 90)) {
    result = 5;
  } else if (vibSpeed >= 90) {
    result = 0;
  }
  return result;
}

// https://stackoverflow.com/questions/9072320/split-string-into-string-array
String getValue(String data, char separator, int index)
{
  int found = 0;
  int strIndex[] = {0, -1};
  int maxIndex = data.length() - 1;

  for (int i = 0; i <= maxIndex && found <= index; i++) {
    if (data.charAt(i) == separator || i == maxIndex) {
      found++;
      strIndex[0] = strIndex[1] + 1;
      strIndex[1] = (i == maxIndex) ? i + 1 : i;
    }
  }

  return found > index ? data.substring(strIndex[0], strIndex[1]) : "";
}


// here is where the magic SHOULD HAPPEN
void timeshareActuate() {
  if ((timeSoFar == 0) || (timeSoFar % (whON + whOFF) == 0))
  {
    digitalWrite(whPin,HIGH);
  } else if ((timeSoFar == whON) || (timeSoFar % (whON + whOFF) == whON)) {
    digitalWrite(whPin,LOW);
  };
/*
  Serial.print(timeSoFar);
  Serial.print("(");
  Serial.print(whON);
  Serial.print("-");
  Serial.print(whOFF);
  Serial.println(")");
  */
  delay(timeSlotMillis);
  timeSoFar += timeSlotMillis;
  if (timeSoFar == 32000) {
    timeSoFar = 0;
  }
}

void readSerial() {
  if (Serial.available() > 0) {
    //if (Serial) {
    //msg = Serial.readString();
    String premsg = Serial.readStringUntil('>');
    msg = Serial.readStringUntil('<');
    String postmsg = Serial.readString();

    //String msg_fl = getValue(msg,'|',0);
    //String msg_fr = getValue(msg,'|',1);
    int msg_wh = getValue(msg, '|', 2).toInt();
    int msg_bk = getValue(msg,'|',3).toInt();
    //String msg_rl = getValue(msg,'|',4);
    //String msg_rr = getValue(msg,'|',5);
    //Serial.println(msg);


    // FRONT LEFT
    // FRONT RIGHT
    // WHEEL
    if (msg_wh <= maxVib and msg_wh >= 0) {
      // subtract the last reading:
      total_wh = total_wh - readings_wh[readIndex_wh];
      // read from the sensor:
      readings_wh[readIndex_wh] = msg_wh;
      // add the reading to the total:
      total_wh = total_wh + readings_wh[readIndex_wh];
      // advance to the next position in the array:
      readIndex_wh = readIndex_wh + 1;

      // if we're at the end of the array...
      if (readIndex_wh >= numReadings) {
        // ...wrap around to the beginning:
        readIndex_wh = 0;
      }
      // calculate the average:
      whVibLevel = total_wh / numReadings;
    }
      // if we're at the end of the array...
      if (readIndex_wh >= numReadings) {
        // ...wrap around to the beginning:
        readIndex_wh = 0;
      }
      // calculate the average, get the OFF value:
      //speed = total_wh / numReadings;
      whOFF = getVib(total_wh / numReadings);

      
    // BACK(SEAT)
    if (msg_bk <= maxVib and msg_bk >= 0) {
      // subtract the last reading:
      total_bk = total_bk - readings_bk[readIndex_bk];
      // read from the sensor:
      readings_bk[readIndex_bk] = msg_bk;
      // add the reading to the total:
      total_bk = total_bk + readings_bk[readIndex_bk];
      // advance to the next position in the array:
      readIndex_bk = readIndex_bk + 1;
      
      // if we're at the end of the array...
      if (readIndex_bk >= numReadings) {
        // ...wrap around to the beginning:
        readIndex_bk = 0;
      }
      // calculate the average, get the OFF value:
      bkOFF = getVib(total_wh / numReadings);
    }

    
    // REAR LEFT
    // REAR RIGHT

    //ACTIONS
    //Serial.println(whVibLevel);
  }
}


void setup()
{
  Serial.begin(115200);
  Serial.setTimeout(SerialTimeout);

  // initialize all the readings to 0:
  for (int thisReading_wh = 0; thisReading_wh < numReadings; thisReading_wh++) {
    readings_wh[thisReading_wh] = 0;
  }
  for (int thisReading_bk = 0; thisReading_bk < numReadings; thisReading_bk++) {
    readings_bk[thisReading_bk] = 0;
  }

  pinMode(whPin,OUTPUT);
  //TEST it works
  whVibLevel = 100;
  actuate(whON, getVib(whVibLevel));
  whVibLevel = 0;
  actuate(whON, getVib(whVibLevel));
  bkVibLevel = 100;
  actuate(bkON, getVib(bkVibLevel));
  bkVibLevel = 0;
  actuate(bkON, getVib(bkVibLevel));
  pinMode(LED_BUILTIN, OUTPUT);
  for (int i = 0; i <= 10; i++) {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(50);
    digitalWrite(LED_BUILTIN, LOW);
    delay(50);
  }
}

void loop()
{
  //readSerialTest();
  readSerial();
  timeshareActuate();

}
