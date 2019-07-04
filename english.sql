-- MySQL dump 10.17  Distrib 10.3.15-MariaDB, for debian-linux-gnu (x86_64)
--
-- Host: localhost    Database: mwenglish
-- ------------------------------------------------------
-- Server version	10.3.15-MariaDB-1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `__diesel_schema_migrations`
--

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `__diesel_schema_migrations`
--

LOCK TABLES `__diesel_schema_migrations` WRITE;
/*!40000 ALTER TABLE `__diesel_schema_migrations` DISABLE KEYS */;
INSERT INTO `__diesel_schema_migrations` VALUES ('20190626084651','2019-07-01 07:56:21');
/*!40000 ALTER TABLE `__diesel_schema_migrations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `exam_manage`
--

DROP TABLE IF EXISTS `exam_manage`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `exam_manage` (
  `id` varchar(32) NOT NULL COMMENT '考试编号',
  `exam_name` varchar(45) NOT NULL COMMENT '考试名称',
  `exam_date` datetime NOT NULL COMMENT '考试时间',
  `exam_end_date` datetime NOT NULL COMMENT '考试结束日期',
  `exam_last_time` int(11) NOT NULL COMMENT '考试持续时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `exam_manage`
--

LOCK TABLES `exam_manage` WRITE;
/*!40000 ALTER TABLE `exam_manage` DISABLE KEYS */;
/*!40000 ALTER TABLE `exam_manage` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `fill_questions`
--

DROP TABLE IF EXISTS `fill_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `fill_questions` (
  `id` varchar(32) NOT NULL COMMENT '问题编号',
  `subject_id` varchar(32) NOT NULL COMMENT '科目编号',
  `question_content` varchar(200) NOT NULL COMMENT '问题内容',
  `corrent_answer` varchar(100) NOT NULL COMMENT '正确答案',
  `value` int(11) NOT NULL COMMENT '分值',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `fill_questions`
--

LOCK TABLES `fill_questions` WRITE;
/*!40000 ALTER TABLE `fill_questions` DISABLE KEYS */;
/*!40000 ALTER TABLE `fill_questions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `paper_manage`
--

DROP TABLE IF EXISTS `paper_manage`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `paper_manage` (
  `id` varchar(32) NOT NULL COMMENT '试卷编号',
  `exam_id` varchar(32) NOT NULL COMMENT '考试编号',
  `subject` varchar(20) NOT NULL COMMENT '科目',
  `question_type_no` int(11) NOT NULL COMMENT '试题类型',
  `question_id` varchar(32) NOT NULL COMMENT '试题编号',
  `question_score` int(11) NOT NULL COMMENT '试题分数',
  PRIMARY KEY (`id`),
  KEY `fk_paper_manage_exam_manage1_idx` (`exam_id`),
  KEY `fk_paper_manage_question_type1_idx` (`question_type_no`),
  KEY `fk_paper_manage_subject_info1_idx` (`subject`),
  CONSTRAINT `fk_paper_manage_exam_manage1` FOREIGN KEY (`exam_id`) REFERENCES `exam_manage` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk_paper_manage_question_type1` FOREIGN KEY (`question_type_no`) REFERENCES `question_type` (`no`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk_paper_manage_subject_info1` FOREIGN KEY (`subject`) REFERENCES `subject_info` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `paper_manage`
--

LOCK TABLES `paper_manage` WRITE;
/*!40000 ALTER TABLE `paper_manage` DISABLE KEYS */;
/*!40000 ALTER TABLE `paper_manage` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `question_type`
--

DROP TABLE IF EXISTS `question_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `question_type` (
  `no` int(11) NOT NULL,
  `type` varchar(45) NOT NULL COMMENT '题库类型\n',
  PRIMARY KEY (`no`),
  UNIQUE KEY `type_UNIQUE` (`type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `question_type`
--

LOCK TABLES `question_type` WRITE;
/*!40000 ALTER TABLE `question_type` DISABLE KEYS */;
/*!40000 ALTER TABLE `question_type` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `score_manage`
--

DROP TABLE IF EXISTS `score_manage`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `score_manage` (
  `id` varchar(32) NOT NULL COMMENT '成绩编号',
  `exam_id` varchar(32) NOT NULL COMMENT '考试编号',
  `user_id` varchar(32) NOT NULL COMMENT '用户编号',
  `subject_id` varchar(32) NOT NULL COMMENT '课程编号',
  `score` int(11) NOT NULL COMMENT '成绩',
  PRIMARY KEY (`id`),
  KEY `fk_score_manage_users1_idx` (`user_id`),
  KEY `fk_score_manage_exam_manage1_idx` (`exam_id`),
  KEY `fk_score_manage_subject_info1_idx` (`subject_id`),
  CONSTRAINT `fk_score_manage_exam_manage1` FOREIGN KEY (`exam_id`) REFERENCES `exam_manage` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT `fk_score_manage_subject_info1` FOREIGN KEY (`subject_id`) REFERENCES `subject_info` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
  CONSTRAINT `fk_score_manage_users1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `score_manage`
--

LOCK TABLES `score_manage` WRITE;
/*!40000 ALTER TABLE `score_manage` DISABLE KEYS */;
/*!40000 ALTER TABLE `score_manage` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `single_questions`
--

DROP TABLE IF EXISTS `single_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `single_questions` (
  `id` varchar(32) NOT NULL COMMENT '问题编号',
  `subject_id` varchar(32) NOT NULL COMMENT '科目编号',
  `question_content` varchar(200) NOT NULL COMMENT '问题内容',
  `answer_a` varchar(100) NOT NULL COMMENT '选项A',
  `answer_b` varchar(100) NOT NULL COMMENT '选项B',
  `answer_c` varchar(100) DEFAULT NULL COMMENT '选项C',
  `answer_d` varchar(100) DEFAULT NULL COMMENT '选项D',
  `corrent_answer` varchar(5) NOT NULL COMMENT '正确选项',
  `value` int(11) NOT NULL COMMENT '分值',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `single_questions`
--

LOCK TABLES `single_questions` WRITE;
/*!40000 ALTER TABLE `single_questions` DISABLE KEYS */;
/*!40000 ALTER TABLE `single_questions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `subject_info`
--

DROP TABLE IF EXISTS `subject_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `subject_info` (
  `id` varchar(32) NOT NULL COMMENT '课程编号',
  `subject` varchar(20) NOT NULL COMMENT '课程名',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `subject_info`
--

LOCK TABLES `subject_info` WRITE;
/*!40000 ALTER TABLE `subject_info` DISABLE KEYS */;
/*!40000 ALTER TABLE `subject_info` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tf_questions`
--

DROP TABLE IF EXISTS `tf_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tf_questions` (
  `id` varchar(32) NOT NULL COMMENT '问题编号\n',
  `subject_id` varchar(32) NOT NULL COMMENT '科目编号',
  `question_content` varchar(200) NOT NULL COMMENT '问题内容',
  `corrent_answer` tinyint(4) NOT NULL COMMENT '正确答案',
  `value` int(11) NOT NULL COMMENT '分值',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tf_questions`
--

LOCK TABLES `tf_questions` WRITE;
/*!40000 ALTER TABLE `tf_questions` DISABLE KEYS */;
/*!40000 ALTER TABLE `tf_questions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `user_session`
--

DROP TABLE IF EXISTS `user_session`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `user_session` (
  `token` varchar(32) NOT NULL,
  `pbkey` varchar(200) NOT NULL,
  `pvkey` varchar(200) NOT NULL,
  `user_id` varchar(32) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `user_session`
--

LOCK TABLES `user_session` WRITE;
/*!40000 ALTER TABLE `user_session` DISABLE KEYS */;
/*!40000 ALTER TABLE `user_session` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` varchar(32) NOT NULL COMMENT '用户编号',
  `phone` varchar(11) NOT NULL COMMENT '用户登录的手机号',
  `password` varchar(50) NOT NULL COMMENT '用户登录密码',
  `nickname` varchar(50) DEFAULT '' COMMENT '显示的用户昵称',
  `avatar` varchar(255) DEFAULT '' COMMENT '头像地址',
  `is_admin` tinyint(4) NOT NULL DEFAULT 0 COMMENT '是否为管理员，0为否1为真',
  `created_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '创建时间',
  `last_modified_at` datetime NOT NULL DEFAULT current_timestamp() COMMENT '上次修改时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `phone_UNIQUE` (`phone`),
  UNIQUE KEY `nickname_UNIQUE` (`nickname`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES ('46cbfaad0f0143c098da81aececed1a9','13224252273','098f6bcd4621d373cade4e832627b4f6','13224252273','',0,'2019-07-02 20:07:08','2019-07-02 20:07:08');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2019-07-03  9:55:19
