<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>makePayment</name>
   <tag></tag>
   <elementGuidId>ca818eec-29a0-4d86-8a32-9413e8fe9093</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;requestId\&quot;: \&quot;611177123\&quot;,\n  \&quot;debitAccount\&quot;: \&quot;0720556019\&quot;,\n  \&quot;isFees\&quot;: false,\n  \&quot;charges\&quot;: [\n    {\n      \&quot;account\&quot;: \&quot;0135582016\&quot;,\n      \&quot;fee\&quot;: 0\n    }\n  ],\n  \&quot;amountPaid\&quot;: 200,\n  \&quot;pid\&quot;: \&quot;N-233333\&quot;,\n  \&quot;agencyCode\&quot;: \&quot;4130001\&quot;,\n  \&quot;revCode\&quot;: \&quot;0008049\&quot;,\n  \&quot;webguid\&quot;: \&quot;2333330-7098216-763\&quot;,\n  \&quot;billType\&quot;: \&quot;webguid\&quot;,\n  \&quot;currency\&quot;: \&quot;NGN\&quot;,\n  \&quot;paymentChannel\&quot;: \&quot;web\&quot;,\n  \&quot;tellerName\&quot;: \&quot;\&quot;,\n  \&quot;bankNote\&quot;: \&quot;\&quot;,\n  \&quot;tellerID\&quot;: \&quot;\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>client_id</name>
      <type>Main</type>
      <value>123</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://revpaymcsvc.fcmb-azr-msase.p.azurewebsites.net/api/RevPay/makePayment</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
